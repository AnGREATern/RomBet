import json
import pandas as pd
import numpy as np
import glob
import os
import argparse
import matplotlib.pyplot as plt
from pathlib import Path

def load_k6_results(results_dir):
    """Загрузка и агрегация результатов k6"""
    all_data = []
    
    # Process each iteration's k6 results
    for file_path in glob.glob(f"{results_dir}/raw/k6_results_*.json"):
        iteration_num = os.path.basename(file_path).split('_')[-1].split('.')[0]
        
        try:
            with open(file_path, 'r') as f:
                # k6 outputs JSON lines format
                for line in f:
                    if line.strip():
                        try:
                            data = json.loads(line)
                            if data.get('type') == 'Point':
                                metric = data.get('metric')
                                # Extract value based on data structure
                                if 'data' in data and 'value' in data['data']:
                                    value = data['data']['value']
                                    timestamp = data.get('data', {}).get('time', None)
                                    
                                    all_data.append({
                                        'iteration': iteration_num,
                                        'metric': metric,
                                        'timestamp': timestamp,
                                        'value': value
                                    })
                        except json.JSONDecodeError:
                            continue
        except Exception as e:
            print(f"Warning: Could not process {file_path}: {e}")
            continue
    
    return pd.DataFrame(all_data)

def load_resource_data(results_dir):
    """Загрузка данных об использовании ресурсов"""
    resource_data = []
    
    # Process system resource data
    for file_path in glob.glob(f"{results_dir}/raw/system_resources_*.txt"):
        iteration_num = os.path.basename(file_path).split('_')[-1].split('.')[0]
        
        try:
            with open(file_path, 'r') as f:
                lines = f.readlines()
                
            for line in lines:
                if 'Average:' in line or line.startswith('Linux'):
                    continue
                
                # Parse sar output
                parts = line.strip().split()
                if len(parts) >= 8 and parts[0].isdigit():  # CPU data
                    resource_data.append({
                        'iteration': iteration_num,
                        'resource_type': 'cpu_user',
                        'value': float(parts[2]) if parts[2] != 'NULL' else 0
                    })
                    resource_data.append({
                        'iteration': iteration_num,
                        'resource_type': 'cpu_system',
                        'value': float(parts[3]) if parts[3] != 'NULL' else 0
                    })
                elif len(parts) >= 5 and 'kbmemfree' not in line:  # Memory data
                    try:
                        mem_used_percent = (float(parts[3]) / (float(parts[2]) + float(parts[3]))) * 100
                        resource_data.append({
                            'iteration': iteration_num,
                            'resource_type': 'memory_percent',
                            'value': mem_used_percent
                        })
                    except:
                        continue
        except Exception as e:
            print(f"Warning: Could not process resource file {file_path}: {e}")
            continue
    
    return pd.DataFrame(resource_data)

def calculate_percentiles(series, metric_name=None):
    """Расчет перцентилей для метрики"""
    if len(series) == 0:
        return {}
    
    return {
        'metric': metric_name,
        'count': len(series),
        'p50': np.percentile(series, 50),
        'p75': np.percentile(series, 75),
        'p90': np.percentile(series, 90),
        'p95': np.percentile(series, 95),
        'p99': np.percentile(series, 99),
        'mean': np.mean(series),
        'std': np.std(series),
        'min': np.min(series),
        'max': np.max(series)
    }

def generate_distribution_chart(df, metric_name, output_path):
    """Генерация графика распределения значений"""
    try:
        values = df[df['metric'] == metric_name]['value']
        if len(values) == 0:
            return
            
        plt.figure(figsize=(10, 6))
        plt.hist(values, bins=50, alpha=0.7, color='skyblue', edgecolor='black')
        plt.title(f'Distribution of {metric_name}')
        plt.xlabel('Value')
        plt.ylabel('Frequency')
        plt.grid(True, alpha=0.3)
        plt.savefig(output_path, dpi=300, bbox_inches='tight')
        plt.close()
    except Exception as e:
        print(f"Warning: Could not generate distribution chart for {metric_name}: {e}")

def generate_time_series_chart(df, metric_name, output_path):
    """Генерация временного ряда значений"""
    try:
        metric_data = df[df['metric'] == metric_name]
        if len(metric_data) == 0:
            return
            
        plt.figure(figsize=(12, 6))
        # Group by iteration and calculate mean for each iteration
        iteration_means = metric_data.groupby('iteration')['value'].mean()
        
        plt.plot(range(len(iteration_means)), iteration_means.values, marker='o', linewidth=2)
        plt.title(f'Time Series of {metric_name} (Mean per Iteration)')
        plt.xlabel('Iteration')
        plt.ylabel('Value')
        plt.grid(True, alpha=0.3)
        plt.savefig(output_path, dpi=300, bbox_inches='tight')
        plt.close()
    except Exception as e:
        print(f"Warning: Could not generate time series chart for {metric_name}: {e}")

def generate_resource_charts(resource_df, results_dir):
    """Генерация графиков использования ресурсов"""
    try:
        if len(resource_df) == 0:
            return
            
        # CPU usage chart
        cpu_data = resource_df[resource_df['resource_type'].isin(['cpu_user', 'cpu_system'])]
        if len(cpu_data) > 0:
            plt.figure(figsize=(12, 6))
            for resource_type in cpu_data['resource_type'].unique():
                data = cpu_data[cpu_data['resource_type'] == resource_type]
                iteration_means = data.groupby('iteration')['value'].mean()
                plt.plot(range(len(iteration_means)), iteration_means.values, 
                        marker='o', label=resource_type, linewidth=2)
            
            plt.title('CPU Usage Over Iterations')
            plt.xlabel('Iteration')
            plt.ylabel('CPU Usage (%)')
            plt.legend()
            plt.grid(True, alpha=0.3)
            plt.savefig(f'{results_dir}/summary/cpu_usage.png', dpi=300, bbox_inches='tight')
            plt.close()
        
        # Memory usage chart
        memory_data = resource_df[resource_df['resource_type'] == 'memory_percent']
        if len(memory_data) > 0:
            plt.figure(figsize=(12, 6))
            iteration_means = memory_data.groupby('iteration')['value'].mean()
            plt.plot(range(len(iteration_means)), iteration_means.values, 
                    marker='o', color='red', linewidth=2)
            
            plt.title('Memory Usage Over Iterations')
            plt.xlabel('Iteration')
            plt.ylabel('Memory Usage (%)')
            plt.grid(True, alpha=0.3)
            plt.savefig(f'{results_dir}/summary/memory_usage.png', dpi=300, bbox_inches='tight')
            plt.close()
    except Exception as e:
        print(f"Warning: Could not generate resource charts: {e}")

def generate_comprehensive_report(results_dir):
    """Генерация комплексного отчета"""
    
    # Load data
    k6_df = load_k6_results(results_dir)
    resource_df = load_resource_data(results_dir)
    
    # Create summary directory if it doesn't exist
    Path(f'{results_dir}/summary').mkdir(parents=True, exist_ok=True)
    Path(f'{results_dir}/summary/charts').mkdir(parents=True, exist_ok=True)
    
    # Calculate metrics
    report = {
        'summary': {
            'total_iterations': len(k6_df['iteration'].unique()) if len(k6_df) > 0 else 0,
            'total_requests': len(k6_df[k6_df['metric'] == 'http_reqs']) if len(k6_df) > 0 else 0
        },
        'performance_metrics': {},
        'resource_metrics': {}
    }
    
    # Performance metrics
    performance_metrics = [
        'http_req_duration', 
        'round_results_response_time', 
        'prepare_round_response_time',
        'http_reqs',
        'errors'
    ]
    
    for metric in performance_metrics:
        if len(k6_df[k6_df['metric'] == metric]) > 0:
            report['performance_metrics'][metric] = calculate_percentiles(
                k6_df[k6_df['metric'] == metric]['value'], 
                metric
            )
    
    # Resource metrics
    if len(resource_df) > 0:
        for resource_type in resource_df['resource_type'].unique():
            values = resource_df[resource_df['resource_type'] == resource_type]['value']
            if len(values) > 0:
                report['resource_metrics'][resource_type] = calculate_percentiles(values, resource_type)
    
    # Generate charts
    if len(k6_df) > 0:
        for metric in ['http_req_duration', 'round_results_response_time']:
            if len(k6_df[k6_df['metric'] == metric]) > 0:
                generate_distribution_chart(
                    k6_df, 
                    metric, 
                    f'{results_dir}/summary/charts/{metric}_distribution.png'
                )
                generate_time_series_chart(
                    k6_df, 
                    metric, 
                    f'{results_dir}/summary/charts/{metric}_timeseries.png'
                )
    
    # Generate resource charts
    generate_resource_charts(resource_df, results_dir)
    
    # Save reports
    with open(f'{results_dir}/summary/final_report.json', 'w') as f:
        json.dump(report, f, indent=2)
    
    # Save detailed CSV data
    if len(k6_df) > 0:
        k6_df.to_csv(f'{results_dir}/summary/all_metrics.csv', index=False)
    
    if len(resource_df) > 0:
        resource_df.to_csv(f'{results_dir}/summary/resource_metrics.csv', index=False)
    
    print("Comprehensive report generated:")
    print(f"  - JSON Report: {results_dir}/summary/final_report.json")
    print(f"  - Performance CSV: {results_dir}/summary/all_metrics.csv")
    print(f"  - Resource CSV: {results_dir}/summary/resource_metrics.csv")
    print(f"  - Charts: {results_dir}/summary/charts/")
    
    return report

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('--results-dir', required=True)
    args = parser.parse_args()
    
    generate_comprehensive_report(args.results_dir)

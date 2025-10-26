import json
import pandas as pd
import numpy as np
import glob
import os
import argparse

def load_k6_results(results_dir):
    """Загрузка и агрегация результатов k6"""
    all_data = []
    
    for file_path in glob.glob(f"{results_dir}/raw/k6_results_*.json"):
        with open(file_path, 'r') as f:
            data = json.load(f)
            for metric_name, metric_data in data['metrics'].items():
                if 'values' in metric_data:
                    for value in metric_data['values']:
                        all_data.append({
                            'iteration': os.path.basename(file_path).split('_')[-1].split('.')[0],
                            'metric': metric_name,
                            'timestamp': value['time'],
                            'value': value['value']
                        })
    
    return pd.DataFrame(all_data)

def calculate_percentiles(df, metric_name):
    """Расчет перцентилей для метрики"""
    values = df[df['metric'] == metric_name]['value']
    return {
        'p50': np.percentile(values, 50),
        'p75': np.percentile(values, 75),
        'p90': np.percentile(values, 90),
        'p95': np.percentile(values, 95),
        'p99': np.percentile(values, 99),
        'mean': np.mean(values),
        'std': np.std(values),
        'min': np.min(values),
        'max': np.max(values)
    }

def generate_report(results_dir):
    """Генерация финального отчета"""
    
    k6_df = load_k6_results(results_dir)
    
    report = {
        'response_times': calculate_percentiles(k6_df, 'http_req_duration'),
        'request_rate': calculate_percentiles(k6_df, 'http_reqs'),
        'error_rate': calculate_percentiles(k6_df, 'errors'),
        'iterations': len(k6_df['iteration'].unique())
    }
    
    with open(f'{results_dir}/summary/final_report.json', 'w') as f:
        json.dump(report, f, indent=2)
    
    k6_df.to_csv(f'{results_dir}/summary/all_metrics.csv', index=False)
    
    print("Final report generated:")
    print(f"  - JSON Report: {results_dir}/summary/final_report.json")
    print(f"  - CSV Data: {results_dir}/summary/all_metrics.csv")
    
    return report

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('--results-dir', required=True)
    args = parser.parse_args()
    
    generate_report(args.results_dir)

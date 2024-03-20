use std::io;
use serde::Deserialize;
use colored::*;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION};
use std::env; // Add this at the top of your file to use the environment variable functionality

#[derive(Deserialize, Debug)]
struct DagList {
    dags: Vec<Dag>,
}

#[derive(Deserialize, Debug)]
struct Dag {
    dag_id: String,
}

#[derive(Deserialize, Debug)]
struct DagRunList {
    dag_runs: Vec<DagRun>,
}

#[derive(Deserialize, Debug)]
struct DagRun {
    dag_run_id: String,
    state: String,
}

fn get_dags(airflow_base_url: &str, username: &str, password: &str) -> Result<DagList, reqwest::Error> {
    let url = format!("{}/api/v1/dags", airflow_base_url);
    let client = Client::new();
    let auth_value = base64::encode(format!("{}:{}", username, password));
    let response = client.get(&url)
        .header(AUTHORIZATION, format!("Basic {}", auth_value))
        .send()?;
    let dag_list = response.json::<DagList>()?;
    Ok(dag_list)
}

fn get_dag_runs(airflow_base_url: &str, username: &str, password: &str, dag_id: &str) -> Result<DagRunList, reqwest::Error> {
    let url = format!("{}/api/v1/dags/{}/dagRuns", airflow_base_url, dag_id);
    let client = Client::new();
    let auth_value = base64::encode(format!("{}:{}", username, password));
    let response = client.get(&url)
        .header(AUTHORIZATION, format!("Basic {}", auth_value))
        .send()?;
    let dag_run_list = response.json::<DagRunList>()?;
    Ok(dag_run_list)
}

fn display_dags(dag_list: &DagList) {
    println!("{}", "DAGs:".bright_yellow());
    for dag in &dag_list.dags {
        println!("> {}", dag.dag_id.bright_blue());
    }
}

fn display_dag_runs(dag_run_list: &DagRunList) {
    println!("{}", "DAG Runs:".bright_yellow());
    for dag_run in &dag_run_list.dag_runs {
        println!("> ID: {}, State: {}", dag_run.dag_run_id.bright_blue(), dag_run.state.bright_green());
    }
}

fn main() {
    println!("{}", "Welcome to Airflow CLI!".bright_yellow());

    let airflow_base_url = "http://localhost:8080"; // Your Airflow instance URL
 
    // Use the `env::var` function to read the AIRFLOW_USER environment variable
    let username = env::var("AIRFLOW_USER").expect("AIRFLOW_USER not set in the environment.");

    // Use the `env::var` function to read the AIRFLOW_PASSWORD environment variable
    let password = env::var("AIRFLOW_PASSWORD").expect("AIRFLOW_PASSWORD not set in the environment.");

    // Fetching and displaying DAGs
    match get_dags(airflow_base_url, &username, &password) {
        Ok(dag_list) => {
            display_dags(&dag_list);

            println!("{}", "Enter the DAG ID to list its runs:".bright_green());
            let mut dag_id = String::new();
            io::stdin().read_line(&mut dag_id).expect("Failed to read input");
            let dag_id = dag_id.trim();

            // Fetching and displaying DAG runs for the specified DAG ID
            match get_dag_runs(airflow_base_url, &username, &password, dag_id) {
                Ok(dag_run_list) => {
                    display_dag_runs(&dag_run_list);
                },
                Err(err) => {
                    eprintln!("Error fetching DAG runs: {}", err);
                }
            }
        },
        Err(err) => {
            eprintln!("Error fetching DAGs: {}", err);
        }
    }
}


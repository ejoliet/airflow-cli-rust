# Airflow CLI Rust

Airflow CLI Rust is a command-line interface (CLI) application written in Rust, designed to interact with Apache Airflow. It allows users to fetch and display a list of Directed Acyclic Graphs (DAGs) and their runs from an Airflow instance. This tool is particularly useful for developers and administrators who need to quickly check the status of Airflow DAGs without accessing the Airflow web UI.

## Description

This CLI tool utilizes Airflow's REST API to fetch information about DAGs and DAG runs. It supports basic authentication to ensure secure access to your Airflow instance. Developed with Rust's powerful concurrency features and safety guarantees, Airflow CLI Rust provides a fast and reliable user experience.

## Features

- List all DAGs available in the specified Airflow instance.
- Fetch and display runs for a specified DAG.
- Supports basic authentication for secure access to Airflow's REST API.

## Getting Started

### Prerequisites

- Rust programming language environment ([installation guide](https://www.rust-lang.org/tools/install))
- Access to an Apache Airflow instance with the REST API enabled

### Installing

1. Clone the repository to your local machine:

```bash
git clone https://github.com/ejoliet/airflow-cli-rust.git
cd airflow-cli-rust
```

2. Build the project using Cargo, Rust's package manager and build system:

```bash
cargo build --release
```

This command compiles the project and generates an executable in `target/release/airflow-cli-rust`.

#### Rust

To install and run a Rust program like the one provided on macOS, you'll need to follow these steps, including the adjustments for basic authentication with Airflow.

1. Install Rust
If you haven't already installed Rust, you can do so by using rustup, which is Rust's official installation method. Open your terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This command downloads and runs the rustup script, which installs the Rust compiler (rustc), the Rust package manager (cargo), and other useful tools. You might need to restart your terminal or source your profile script to ensure that the Rust tools are in your PATH.

### Configuration

Before running the CLI, ensure you have the URL of your Airflow instance and the credentials for basic authentication. The tool expects these as inputs during runtime.

### Running the Application

#### user and password

You'll need to setup the environment variable to authenticate against Airflow REST API.

```bash
export AIRFLOW_PASSWORD=your_actual_password_here
export AIRFLOW_USER=your_actual_user_here
```

To start the CLI application, use the following command:

```bash
cargo run --release
```

Follow the on-screen prompts to interact with your Airflow instance.

## Dependencies

This project relies on several external crates, including:

- `reqwest` for making HTTP requests.
- `serde` and `serde_json` for JSON serialization and deserialization.
- `colored` for coloring terminal output.
- `tokio` for async runtime.

Dependencies are managed by Cargo and defined in the `Cargo.toml` file.

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".

Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Acknowledgements

- [Rust Programming Language](https://www.rust-lang.org/)
- [Apache Airflow](https://airflow.apache.org/)

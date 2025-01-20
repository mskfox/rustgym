# RustGym

RustGym is a comprehensive tool for coding problem enthusiasts and developers who want to practice problem-solving in Rust. It not only provides a collection of problem solutions but also offers a powerful CLI to search, filter, and test problems. Whether you’re looking to explore problems by various criteria or execute tests on specific solutions, RustGym facilitates an organized and efficient workflow.

## Key Features

- **Problem Search**: Filter problems by name, ID, difficulty, or platform to find exactly what you're looking for.
- **Pre-built Solutions**: Contains solutions for numerous coding problems across various platforms.
- **Run Tests**: Easily run tests for specific problems directly from the command line using your local environment.
- **Well-structured CLI**: Utilize commands to streamline your workflow for searching and testing problems.
- **Configurable Workflow**: The ``rustgym.toml`` configuration file allows you to easily define problem details and access them quickly.

## Installation Instructions

1. Clone the repository:
   ```bash
   git clone https://github.com/mskfox/rustgym.git
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```

## Configuration

RustGym uses a configuration file named ``rustgym.toml`` to define problem details in an easy-to-manage format. This file allows you to specify the problem’s name, ID, platform, difficulty, and any relevant URLs.

### Example:

```toml
name = "Two Sum"
id = "1"
platform = "leetcode"
difficulty = "easy"
url = "<optional url override>"
```

## Usage

### Search Command

Use the search functionality to filter problems by different criteria:

```bash
rustgym search [--name <name>] [--id <id>] [--difficulty <difficulty>] [--platform <platform>]
```

#### Options:
- `--name`: Search by problem name (optional)
- `--id`: Search by problem ID (optional)
- `--difficulty`: Filter by difficulty (easy, medium, hard)
- `--platform`: Filter by platform (e.g., LeetCode, Codeforces)

### Test Command

Run tests for a specific problem by providing search criteria:

```bash
rustgym test [--name <name>] [--id <id>] [--platform <platform>]
```

This will locate the problem, execute its tests, and provide feedback on whether they pass or fail.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
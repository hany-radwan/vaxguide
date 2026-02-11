# VaxGuide

VaxGuide is a command-line application written in Rust that provides general vaccination guidance based on public health recommendations in Germany and the European Union context.

This tool is designed for educational and academic purposes within the Digital Business Skills module.

---

## Project Overview

VaxGuide collects basic user information and generates a personalized vaccination guidance summary based on:

- Age
- Presence of chronic medical conditions
- Healthcare worker or medical student status
- Pregnancy status
- Exposure to Bavaria (Bayern) and outdoor activities

The application uses simplified decision logic inspired by public vaccination guidelines.

---

## Features

- Interactive CLI input
- Conditional recommendation logic
- Public health–based reasoning explanations
- Region-specific FSME (Tick-Borne Encephalitis) logic
- Occupational vaccination logic (Hepatitis B)
- Clear summary output
- Informational disclaimer
- Optional commercial extension hint (clinic finder link)

---

## Implemented Vaccination Logic

The tool may generate recommendations related to:

- Influenza (Seasonal Flu)
- COVID-19 Booster
- Tetanus / Diphtheria / Pertussis (Tdap)
- Hepatitis B (for healthcare workers)
- FSME (Tick-Borne Encephalitis) for Bavaria exposure

All recommendations are informational and not diagnostic.

---

## Technology Stack

- Language: Rust
- Build system: Cargo
- Interface: Terminal / CLI

---

## How to Run the Project

### 1. Clone the repository

```bash
git clone https://github.com/Hany-Radwan/VaxGuide.git
cd VaxGuide
```

### 2. Build the project

```bash
cargo build
```

### 3. Run the application

```bash
cargo run
```

---

## Example Interaction

```
Enter your age (years): 46
Do you have a chronic medical condition? (y/n): y
Are you a healthcare worker or medical student? (y/n): y
Are you currently pregnant? (y/n): n
Will you live, work, or spend significant outdoor time in Bavaria (Bayern)? (y/n): y
```

The tool then generates a personalized vaccination guidance summary.

---

## Disclaimer

This tool provides general public-health information based on published guidelines.

It does not provide medical diagnosis or replace professional medical advice.

Users should consult a qualified healthcare professional for personal medical decisions.

---

## Academic Context

This project was developed as part of the Digital Business Skills module at Deggendorf Institute of Technology.

It demonstrates:

- Basic Rust programming
- Conditional logic implementation
- CLI interaction design
- Health informatics reasoning
- Simple productization concept (clinic finder extension)

---

## Author

Hany Radwan  
Health Informatics Student  
Deggendorf Institute of Technology

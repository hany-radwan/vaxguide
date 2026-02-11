use std::io::{self, Write};

// Handle user input safely and consistently
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase()
}

// Normalize yes/no answers
fn yes(answer: &str) -> bool {
    answer == "yes" || answer == "y"
}

fn main() {
    // Branding & positioning (exam-safe)
    println!("==============================================");
    println!("   VaxGuide | General Vaccination Guidance     ");
    println!("   Public Health–Based | Germany & EU Context  ");
    println!("==============================================\n");

    // --- Collect user input ---
    let age_input = read_input(">> Enter your age (years): ");
    let age: u32 = age_input.parse().unwrap_or(0);

    let chronic = yes(&read_input(
        ">> Do you have a chronic medical condition? (y/n): ",
    ));

    let healthcare_worker = yes(&read_input(
        ">> Are you a healthcare worker or medical student? (y/n): ",
    ));

    let pregnant = yes(&read_input(
        ">> Are you currently pregnant? (y/n): ",
    ));

    // Regional exposure logic (FSME)
    let bavaria = yes(&read_input(
        ">> Will you live, work, or spend significant outdoor time in Bavaria (Bayern)? (y/n): ",
    ));

    // --- Output header ---
    println!("\nAnalyzing profile using public health vaccination guidelines...\n");
    println!("--------------------------------------------------------------");
    println!("          PERSONALIZED VACCINATION GUIDANCE SUMMARY           ");
    println!("--------------------------------------------------------------\n");

    // --- Vaccination logic ---

    // Influenza
    if age >= 60 || chronic || pregnant || healthcare_worker {
        println!("• Influenza (Seasonal Flu)");
        println!("  Recommendation: Strongly recommended");
        println!("  Reason: Higher risk of complications in identified groups.\n");
    }

    // COVID-19
    if age >= 18 || chronic || pregnant || healthcare_worker {
        println!("• COVID-19 Booster");
        println!("  Recommendation: Recommended according to public health guidance");
        println!("  Reason: Booster doses help maintain protection against severe disease.\n");
    }

    // Tetanus / Diphtheria / Pertussis
    if age >= 18 {
        println!("• Tetanus / Diphtheria / Pertussis (Tdap)");
        println!("  Recommendation: Routine adult booster");
        println!("  Reason: Booster typically recommended every 10 years.\n");
    }

    // Hepatitis B (Occupational Health)
    if healthcare_worker {
        println!("• Hepatitis B");
        println!("  Recommendation: Strongly recommended for occupational exposure");
        println!("  Reason: Common recommendation for clinical training and hospital work.\n");
    }

    // FSME (Tick-Borne Encephalitis)
    if bavaria {
        println!("• FSME (Tick-Borne Encephalitis)");
        println!("  Recommendation: Recommended based on regional exposure");
        println!("  Reason: Increased risk in endemic regions such as parts of Bavaria.");
        println!("  Note: Especially relevant for outdoor activities such as hiking.\n");
    } else {
        println!("• FSME (Tick-Borne Encephalitis)");
        println!("  Recommendation: Not routinely indicated based on selected region.\n");
    }

    // --- Commercial / digital business hint (exam-safe) ---
    println!("--------------------------------------------------------------");
    println!("NEXT STEPS (Optional):");
    println!("• Save this summary for personal records or administrative use.");
    println!("• Consult a healthcare professional to review your vaccination status.");
    println!("• Find verified English-speaking clinics near you: [ www.vaxguide.eu/clinics ]");

    // --- Disclaimer ---
    println!("\nDisclaimer:");
    println!("This tool provides general public-health information based on published guidelines.");
    println!("It does not provide medical diagnosis or replace professional medical advice.");
}

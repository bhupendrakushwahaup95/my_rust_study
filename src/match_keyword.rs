// File: keyword_03_match_master.rs
// Purpose: Mastery of the 'match' keyword in Rust across 4 distinct phases
// Standards: Phase 1 (Normal), Phase 2 (Nested), Phase 3 (Expression), Phase 4 (Advanced)

fn main() {
    println!("============================================================");
    println!("--- KEYWORD 03: 'MATCH' - THE POWERHOUSE OF RUST LOGIC ---");
    println!("============================================================\n");

    // --- PHASE 1: NORMAL (Simple Pattern Matching) ---
    // Determining exam stages based on a numeric value
    println!("--- PHASE 1: Normal Match Usage ---");
    let exam_stage = 2;

    print!("Current Focus: ");
    match exam_stage {
        1 => println!("Tier 1: Focus on Speed and Accuracy in Basics."),
        2 => println!("Tier 2: Focus on Advanced Concepts and Mains Level."),
        3 => println!("Tier 3: Skill Test and Computer Efficiency."),
        // '_' is the catch-all pattern for any value not listed above
        _ => println!("Unknown Stage. Please check your exam notification."),
    }
    println!("------------------------------------------------------------\n");


    // --- PHASE 2: NESTED (Match within a Match) ---
    // Allocation of departments based on State and Category
    println!("--- PHASE 2: Nested Match Logic ---");
    let state_code = "UP";
    let vacancy_type = "Group B";

    print!("Allocation Status: ");
    match state_code {
        "UP" => {
            // Nested match to refine the selection within the state
            match vacancy_type {
                "Group B" => println!("Posted as Assistant Section Officer in Lucknow."),
                "Group C" => println!("Posted as Tax Assistant in Kanpur."),
                _ => println!("General vacancy allocation in Uttar Pradesh."),
            }
        },
        "DL" => {
            match vacancy_type {
                "Group B" => println!("Posted in Ministry Headquarters, New Delhi."),
                _ => println!("Central Secretariat Service allocation."),
            }
        },
        _ => println!("Waitlisted for Other State Cadre allocation."),
    }
    println!("------------------------------------------------------------\n");


    // --- PHASE 3: EXPRESSION (Assigning Value directly) ---
    // Assigning performance remarks based on mock test percentile
    println!("--- PHASE 3: Match as an Expression ---");
    let percentile: u32 = 92;

    // The entire match block returns a &str which is stored in 'remark'
    let remark = match percentile {
        99..=100 => "Exceptional - AIR 1 Prospect",
        90..=98  => "Top Ranker - High chance of desired post",
        75..=89  => "Good Scorer - Safe for selection",
        _        => "Needs more practice and revision", // Default case is mandatory
    };

    println!("Mock Test Result: {} Percentile", percentile);
    println!("Feedback: {}", remark);
    println!("------------------------------------------------------------\n");


    // --- PHASE 4: ADVANCED (Guards and Multiple Patterns) ---
    // Handling complex eligibility using Guards (if) and OR patterns (|)
    println!("--- PHASE 4: Advanced Patterns (Guards & OR) ---");
    let age = 29;
    let has_phd = false;

    print!("Special Eligibility Check: ");
    match age {
        // Multiple patterns using '|'
        18 | 19 | 20 => println!("Eligible for Lower Division Clerk (LDC)."),
        
        // Pattern with a 'Guard' (if condition)
        21..=30 if has_phd => {
            println!("Eligible for Specialized Research Officer posts.");
        },
        21..=30 if !has_phd => {
            println!("Eligible for Executive Assistant / Inspector posts.");
        },
        
        31..=32 => println!("Eligible only for posts with age relaxation."),
        _ => println!("Age criteria not met for general recruitment."),
    }

    // --- FINAL COMPREHENSIVE SYSTEM CHECK ---
    println!("\n--- SYSTEM FINAL INTEGRITY CHECK ---");
    let test_results = [10, 45, 80];
    
    for score in test_results {
        let category = match score {
            s if s < 33 => "Fail",
            33..=60 => "Average",
            _ => "Excellent",
        };
        println!("Score {}: Evaluated as {}", score, category);
    }

    println!("\n============================================================");
    println!("--- END OF MATCH KEYWORD MODULE ---");
    println!("============================================================");
}
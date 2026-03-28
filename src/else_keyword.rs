// File: else_keyword.rs
// Purpose: Deep dive into 'else' and 'else if' keywords within Rust
// Standards: Phase 1 (Normal), Phase 2 (Nested), Phase 3 (Expression), Phase 4 (Advanced)

fn main() {
    println!("============================================================");
    println!("--- KEYWORD 02: 'ELSE' - LOGIC AND FALLBACK SYSTEM ---");
    println!("============================================================\n");

    // --- PHASE 1: NORMAL (The Basic Alternative) ---
    // Determining if a student is Passed or Failed based on a threshold
    println!("--- PHASE 1: Normal Else Usage ---");
    let achieved_marks = 32;
    let passing_threshold = 33;

    print!("Result Status: ");
    if achieved_marks >= passing_threshold {
        println!("Pass. You are eligible for the next level.");
    } else {
        // This block executes because the IF condition (32 >= 33) is false.
        println!("Fail. You need {} more marks to pass.", passing_threshold - achieved_marks);
    }
    println!("------------------------------------------------------------\n");


    // --- PHASE 2: NESTED (The Else-If Ladder) ---
    // Complex Post Allocation Logic based on SSC CGL Rank
    println!("--- PHASE 2: Nested Else-If Ladder ---");
    let all_india_rank = 145;

    print!("Post Allocation: ");
    if all_india_rank <= 50 {
        println!("Selected for MEA (Ministry of External Affairs).");
    } else if all_india_rank <= 200 {
        // This block handles the middle range
        println!("Selected for Income Tax Inspector (ITI).");
    } else if all_india_rank <= 500 {
        println!("Selected for Central Excise Inspector.");
    } else {
        // The final fallback if none of the above specific ranks match
        println!("Selected for Auditor / Tax Assistant posts.");
    }
    println!("------------------------------------------------------------\n");


    // --- PHASE 3: EXPRESSION (Mandatory Else in Assignment) ---
    // Assigning a string based on a boolean flag
    println!("--- PHASE 3: Conditional Assignment via Else ---");
    let is_resident_of_india = true;

    // In a conditional expression, 'else' is mandatory to ensure a value is returned.
    let nationality_status = if is_resident_of_india {
        "Indian Citizen"
    } else {
        "Non-Resident / Foreigner"
    };

    println!("Verification Result: Candidate is an {}", nationality_status);
    println!("------------------------------------------------------------\n");


    // --- PHASE 4: ADVANCED (Handling Missing Data with Else) ---
    // Using else with 'if let' for robust error handling
    println!("--- PHASE 4: Advanced Fallback (if let ... else) ---");
    let candidate_email: Option<&str> = None; // Simulating a case where email is not provided

    print!("Communication Check: ");
    if let Some(email) = candidate_email {
        println!("Notification sent successfully to: {}", email);
    } else {
        // This else block acts as a safety net for 'None' values
        println!("Warning: Email not found! Reverting to SMS notification.");
        
        let phone_number = "98xxxxxx10";
        println!("Action: SMS sent to registered mobile: {}", phone_number);
    }

    // --- SYSTEM FINAL INTEGRITY CHECK (Extended Logic) ---
    println!("\n--- FINAL SYSTEM AUDIT ---");
    let server_response_code = 404;

    if server_response_code == 200 {
        println!("Audit: System Synchronization Successful.");
    } else if server_response_code == 404 {
        println!("Audit: Resource Not Found. Please check the URL.");
    } else {
        println!("Audit: Unknown System Error Detected.");
    }

    println!("============================================================");
    println!("--- END OF ELSE KEYWORD MODULE ---");
    println!("============================================================");
}
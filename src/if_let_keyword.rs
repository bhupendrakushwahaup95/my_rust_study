// File: keyword_04_let_conditional_master.rs
// Purpose: Mastering 'if let' and 'while let' for conditional pattern matching
// Standards: Phase 1 (Normal), Phase 2 (Nested), Phase 3 (Expression), Phase 4 (Advanced)

fn main() {
    println!("============================================================");
    println!("--- KEYWORD 04: 'LET' - CONDITIONAL PATTERN MATCHING ---");
    println!("============================================================\n");

    // --- PHASE 1: NORMAL (Basic if let) ---
    // Extracting a value from an Option safely
    println!("--- PHASE 1: Normal 'if let' Usage ---");
    let exam_center_code: Option<i32> = Some(5012);

    // Instead of using match with a default case, we use if let for simplicity
    if let Some(code) = exam_center_code {
        println!("Center Verified! Your exam center code is: {}", code);
    } else {
        println!("Alert: Exam center not yet allocated.");
    }
    println!("------------------------------------------------------------\n");


    // --- PHASE 2: NESTED (Layered Extraction) ---
    // Handling nested Option data for candidate details
    println!("--- PHASE 2: Nested 'if let' Logic ---");
    let candidate_info: Option<Option<&str>> = Some(Some("Qualified"));

    print!("Verification Status: ");
    if let Some(inner_info) = candidate_info {
        // Nested if let to dig deeper into the data structure
        if let Some(status) = inner_info {
            println!("Final Result Extracted: {}", status);
        } else {
            println!("Inner data is empty.");
        }
    } else {
        println!("No candidate information found.");
    }
    println!("------------------------------------------------------------\n");


    // --- PHASE 3: EXPRESSION (Assignment using if let) ---
    // Assigning a security level based on an optional access token
    println!("--- PHASE 3: Assignment via 'if let' Expression ---");
    let access_token: Option<&str> = Some("ADMIN_SECRET_789");

    // if let can return a value which is then stored in 'security_clearance'
    let security_clearance = if let Some(token) = access_token {
        if token.contains("ADMIN") { "High Level Access" } else { "Standard Access" }
    } else {
        "Guest Access" // Fallback if pattern does not match
    };

    println!("System Access Result: {}", security_clearance);
    println!("------------------------------------------------------------\n");


    // --- PHASE 4: ADVANCED (while let and Variable Shadowing) ---
    // Using while let to drain a stack of exam papers
    println!("--- PHASE 4: Advanced 'while let' and Shadowing ---");
    let mut exam_papers = vec!["Maths", "English", "Reasoning", "GS"];

    println!("Processing Exam Papers:");
    // while let keeps running as long as pop() returns Some(paper)
    while let Some(paper) = exam_papers.pop() {
        // Demonstrating Shadowing: 'paper' is shadowed with uppercase version
        let paper = paper.to_uppercase();
        println!(" - Grading Paper: {}", paper);
    }

    // Example of shadowing within an if block
    let points = Some(100);
    if let Some(points) = points {
        // This 'points' shadows the outer 'points' only within this block
        println!("\nBonus points awarded: {}", points + 50);
    }

    // --- FINAL SYSTEM INTEGRITY CHECK ---
    println!("\n--- SYSTEM FINAL AUDIT ---");
    let system_logs = vec![Some("OK"), None, Some("ERROR")];
    
    for log in system_logs {
        if let Some(msg) = log {
            println!("Log Entry: System is reporting '{}'", msg);
        } else {
            println!("Log Entry: No data found (Null Entry)");
        }
    }

    println!("\n============================================================");
    println!("--- END OF LET KEYWORD MODULE ---");
    println!("============================================================");
}
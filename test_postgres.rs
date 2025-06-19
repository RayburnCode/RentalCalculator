use api::auth::{connect_to_database, User};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();
    
    println!("Attempting to connect to PostgreSQL database...");
    
    // Test database connection
    let pool = connect_to_database().await;
    println!("✅ Successfully connected to PostgreSQL database!");
    
    // Test creating tables
    println!("Creating user tables...");
    User::create_user_tables(&pool).await;
    println!("✅ Successfully created user tables!");
    
    // Test retrieving a user
    println!("Testing user retrieval...");
    match User::get_user(1, &pool).await {
        Some(user) => {
            println!("✅ Successfully retrieved user: {} (ID: {}, Anonymous: {})", 
                user.username, user.id, user.anonymous);
        }
        None => {
            println!("❌ Failed to retrieve user with ID 1");
        }
    }
    
    println!("🎉 All PostgreSQL tests passed!");
    
    Ok(())
}

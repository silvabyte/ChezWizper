use anyhow::Result;
use chezwizper::ui::Indicator;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔊 Testing UI sound methods directly");
    
    let indicator = Indicator::new().with_audio_feedback(true);
    
    println!("Testing recording sound...");
    indicator.show_recording().await?;
    
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    println!("Testing processing sound...");  
    indicator.show_processing().await?;
    
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    println!("Testing completion sound...");
    indicator.show_complete("Test transcription result").await?;
    
    println!("✅ All tests complete!");
    
    Ok(())
}
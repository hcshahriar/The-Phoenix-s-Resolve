/*
 *  PHOENIX'S RESOLVE - A Dark Motivational Engine
 *  
 *  When shadows lengthen and failure looms,
 *  this code ignites the embers of resurgence.
 *  Not just a program - an algorithmic rebirith.
 */

use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Adversity {
    failure_count: u32,
    last_setback: Option<SystemTime>,
    darkest_hour: bool,
}

impl Adversity {
    fn new() -> Self {
        Adversity {
            failure_count: 0,
            last_setback: None,
            darkest_hour: false,
        }
    }

    fn record_setback(&mut self) {
        self.failure_count += 1;
        self.last_setback = Some(SystemTime::now());
        
        if self.failure_count >= 3 {
            self.darkest_hour = true;
        }
    }
}

trait Resurgence {
    fn forge_resolve(&self) -> String;
    fn ashes_to_ember(&self) -> bool;
}

impl Resurgence for Adversity {
    fn forge_resolve(&self) -> String {
        let mut resolve = match self.failure_count {
            0 => "The battle hasn't yet begun".to_string(),
            1 => "First blood - let it fuel your rage".to_string(),
            2 => "The wound deepens - sharpen your will".to_string(),
            3..=5 => format!("{} strikes down - rise {} times", 
                            self.failure_count, 
                            self.failure_count + 1),
            _ => "The abyss stares back - now show it your teeth".to_string(),
        };

        if self.darkest_hour {
            resolve.push_str("\n[THE PHOENIX AWAITS IN THE SHADOWS]");
        }

        resolve
    }

    fn ashes_to_ember(&self) -> bool {
        self.darkest_hour && self.failure_count > 0
    }
}

fn phoenix_meditation(adversity: &Adversity) {
    let mantra = r"
        ||||||||||||||||||||||||||||||||||||
        ||  WHAT BURNS NEVER TRULY DIES   ||
        ||  THE SCARS MAP THE PATH BACK   ||
        ||  EACH END A DISGUISED ORIGIN   ||
        ||||||||||||||||||||||||||||||||||||
    ";

    println!("{}", mantra);
    
    if adversity.ashes_to_ember() {
        println!("[SYSTEM] Critical failure threshold reached");
        println!("[SYSTEM] Initiating resurgence protocol...");
        
        for i in (1..=3).rev() {
            println!("[IGNITION IN {}]", i);
            thread::sleep(Duration::from_secs(1));
        }
        
        println!("[PHOENIX PROTOCOL ACTIVATED]");
    }
}

fn main() {
    let mut battle_scars = Adversity::new();
    
    // Simulate a series of setbacks
    for _ in 0..4 {
        battle_scars.record_setback();
        println!("{}\n", battle_scars.forge_resolve());
        thread::sleep(Duration::from_secs(2));
    }
    
    phoenix_meditation(&battle_scars);
    
    println!("\n[END TRANSMISSION]");
}

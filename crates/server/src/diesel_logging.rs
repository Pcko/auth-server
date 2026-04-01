use diesel::connection::{Instrumentation, InstrumentationEvent};

pub fn diesel_logger() -> Option<Box<dyn Instrumentation>> {
    Some(Box::new(|event: InstrumentationEvent<'_>| match event {
        InstrumentationEvent::StartQuery { query, .. } => {
            println!("SQL : {}", query);
        }
        InstrumentationEvent::FinishQuery {
            error: Some(error), ..
        } => {
            println!("Error : {}", error);
        }
        _ => {}
    }))
}

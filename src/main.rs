use chrono::NaiveDate;

struct Plan {
        line: u32,
        product_code: String,
        product_name: String,
        volume: f32,
        planned_date: NaiveDate,
    }

fn main() {
    
    
    let plan1 = Plan {
        line: 5810,
        product_code: String::from("1010028761"),
        product_name: String::from("Бул.для бургера кунжут 70г"),
        volume: 430.00,
        planned_date: NaiveDate::from_ymd_opt(2026, 10, 09).unwrap(),
    };
    println!("{}", get_plan(&plan1));
    

}

fn get_plan(plan: &Plan) -> String {
    format!(
    "line: {}, code: {}, name: {}, volume: {}, date: {}",
    plan.line,
    plan.product_code,
    plan.product_name,
    plan.volume,
    plan.planned_date
    )
} 
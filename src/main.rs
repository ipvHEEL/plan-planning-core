struct Plan {
        line: u32,
        product_Code: String,
        product_Name: String,
        volume: f32
    }

fn main() {
    
    
    let plan1 = Plan {
        line: 5810,
        product_Code: String::from("1010028761"),
        product_Name: String::from("Бул.для бургера кунжут 70г"),
        volume: 430.00,        
    };
    println!("{}", get_plan(&plan1));


}

fn get_plan(plan: &Plan) -> String {
    format!(
    "line: {}, code {}",
    plan.line,
    plan.product_Code
    )
}
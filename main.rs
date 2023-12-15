use mysql::*;
use mysql::prelude::Queryable;
use mysql::Value::*;
use rand::Rng;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc,NaiveDate};
use mysql::chrono::SubsecRound;

#[derive(Debug, PartialEq, Eq)]

struct Product {
    Pid : String,
    PName: String,
    PType: String,
    RetailPrice: i32,
    Brand: String,
    DiscountPrice: i32,
    Rating: String,
    Status: String
}

struct Orders{
    Oid: String,
    Pid: String,
    Cid: String,
    Date: String
}

fn init_db()-> mysql::PooledConn{
    let url: &str = "mysql://root:Password@password@127.0.0.1:3306/productdb";
    let pool = Pool::new(url).unwrap();
    let conn = pool.get_conn().unwrap();
    return conn;
} 

impl Orders{
    fn place_order(cid:&str){
        let num:i64 = rand::thread_rng().gen_range(0..20000);
        let oid = "oid".to_owned()+&num.to_string();
        let mut prod_id = String::new();
        println!("Enter Product id :");
        std::io::stdin().read_line(&mut prod_id).unwrap();
        prod_id = prod_id.trim().to_owned();
        let dt : DateTime<Utc> = Utc::now().round_subsecs(0);
        let datetime= &dt.to_string().replace("UTC","").replace("T"," ").replace("Z",".00");
        println!("\nDo you want confirm the order <yes/no>: ");
        let mut ch = String::new();
        std::io::stdin().read_line(&mut ch).unwrap();
        let choice = ch.trim().to_lowercase();
        if choice == "yes" {
            let mut conn = init_db();
            let qr = format!("INSERT INTO Orders(Oid,Pid,Cid,Date) VALUES({:?},{:?},{:?},{:?})",&oid,&prod_id,&cid,&datetime);
        
            conn.query::<String, std::string::String>(qr).unwrap();
            println!("Order Confirmed!!");
            println!("\n\t\t*** Order Details ***");
            println!("Order Id : {:?}",&oid);
            println!("Product Id : {:?}",&prod_id);
            println!("Customer Id : {:?}",&cid);
            println!("Date & Time: {:?}",&dt);
        }
    }
    fn view_Orders(){
        let mut conn = init_db();

        let qr = format!("select * from Orders");
        let mut res = conn.query_map(qr,
            |(Oid, Pid, Cid, date,): (std::string::String, std::string::String, std::string::String, _)
            |Orders{
                Oid:Oid,
                Pid: Pid,
                Cid: Cid,
                Date:date,
            },
        ).expect("Query failed");    
        println!("Oid          Pid         Cid     Date");
        for r in res{
            println!(" \n{:?}   {:?}   {:?}   {:?}",r.Oid,r.Pid,r.Cid,r.Date);
        }
    }

    fn customer_analysis(){
        let mut conn = init_db();
        println!("\n\t\t*** Product Customers ***");
        let mut prod_type:[String;10] = ["Beauty and Personal Care".to_owned(),"Clothing".to_owned(),
        "Automotive".to_owned(),"Sports & Fitness".to_owned(),
        "Bags, Wallets & Belts".to_owned(),"Watches".to_owned(),
        "Cameras & Accessories".to_owned(),"Mobiles & Accessories".to_owned(),
        "Furniture".to_owned(),"Computers Components".to_owned()
        ];
        println!("\tProduct Type                | Customer Count ");
        println!("--------------------------------------------");
            
        for pt in &prod_type {
            let mut qr = format!("select count(o.pid) FROM Orders o Where o.pid in (select pid FROM products Where PType={:?});",&pt);
            let mut res:Vec::<i64> = conn.query(qr).unwrap();
            println!("{:?}    | {:?} ", pt, res);
            
        }

        println!("\nSpecify the Product type to analyse..");
        let p_type = Product::get_product_Type();
        println!("\n\t\t***Customers of each {:?} Brands ***",&p_type);
        let mut qr = format!("select Distinct(Brand) FROM Products Where PType={:?};",&p_type);
        let res: Vec<String>= conn.query(qr).unwrap();
        println!("Brand Name           | Customer Count ");
        println!("--------------------------------------");
        for r in res {
            let mut qr = format!("select count(o.pid) FROM Orders o Where o.pid in (select pid FROM Products Where PType={:?} and Brand={:?})",&p_type,&r);
            let res2:Vec::<i64> = conn.query(qr).unwrap();
            println!("{:?}    | {:?} ", r, res2);
        }

    }
}


impl Product{

    fn get_product_Type()->String{
        println!("\n1.Beauty and Personal Care\n2.Clothing\n3.Automotive\n4.Sports & Fitness\n5.Bags, Wallets & Belts\n6.Watches\n7.Cameras & Accessories\n8.Mobiles & Accessories\n9.Furniture\n10.Computers Components");
        
        println!("\nSelect the Product Type:");
        let mut ch = String::new();
        std::io::stdin().read_line(&mut ch).unwrap();
        let choice: i32 = ch.trim().parse().expect("enter valid number");
        let mut prod_type = String::new();
        prod_type = match choice{
            1 => "Beauty and Personal Care".to_owned(),
            2 => "Clothing".to_owned(),
            3 => "Automotive".to_owned(),
            4 => "Sports & Fitness".to_owned(),
            5 => "Bags, Wallets & Belts".to_owned(),
            6 => "Watches".to_owned(),
            7 => "Cameras & Accessories".to_owned(),
            8 => "Mobiles & Accessories".to_owned(),
            9 => "Furniture".to_owned(),
            10 => "Computers Components".to_owned(),
            i32::MIN..=0_i32 | 11_i32..=i32::MAX => todo!(),
        };

        return prod_type
    }
    fn view_products(Cid:&str){
        let prod_type = Self::get_product_Type();
        println!("\n\n\t\t Here are some {:?} Products",&prod_type);
        let mut conn = init_db();
        let qr = format!("select * from Products where PType = {:?}",&prod_type);
        let mut res = conn.query_map(qr,
            |(Pid, PName, PType, RetailPrice, Brand, DiscountPrice,Rating, Status,)
            |Product{
                Pid : Pid,
                PName: PName,
                PType: PType,
                RetailPrice: RetailPrice,
                Brand: Brand,
                DiscountPrice: DiscountPrice,
                Rating:Rating,
                Status: Status,
            },
        ).expect("Query failed");    
        println!("Pid           PName                      PType    RetailPrice     Brand         DiscountPrice   Rating   Status");
        for r in res{
            println!(" \n{:?}   {:?}   {:?}   {:?}   {:?}   {:?}   {:?}   {:?}",
            r.Pid,r.PName,r.PType,r.RetailPrice,r.Brand,r.DiscountPrice,r.Rating,r.Status);
        }
        println!("\nDo you want Buy any product <yes/no>: ");
        let mut ch = String::new();
        std::io::stdin().read_line(&mut ch).unwrap();
        let choice = ch.trim().to_lowercase();
        if choice == "yes" {
        Orders::place_order(Cid);
        }
    }    

    fn add_product(){

        println!("\n\t Enter the Product Details:");
        let num:i64 = rand::thread_rng().gen_range(0..20000);
    
        let pid = "Pid".to_owned()+&num.to_string();
        let mut conn = init_db();

        let mut pname = String::new();
        println!("Enter the name of the item:");
        std::io::stdin().read_line(&mut pname).unwrap();

        let mut ptype = String::new();
        println!("Enter the type of the item:");
        std::io::stdin().read_line(&mut ptype).unwrap();

        let mut rprice = String::new();
        println!("Enter the Retail Price:");
        std::io::stdin().read_line(&mut rprice).unwrap();
        let retailprice:i64 = rprice.trim().parse().expect("enter valid quantity");

        let mut brand = String::new();
        println!("Enter the brand name of the item:");
        std::io::stdin().read_line(&mut brand).unwrap();

        let mut discount = String::new();
        println!("Enter the Discount price:");
        std::io::stdin().read_line(&mut discount).unwrap();
        let discountprice:i64 = discount.trim().parse().expect("enter valid quantity");
            
        let mut rating = String::new();
        println!("Enter the rating:");
        std::io::stdin().read_line(&mut rating).unwrap();

        let mut status = String::new();
        println!("Enter the Product status:");
        std::io::stdin().read_line(&mut status).unwrap();
            
        let qr = format!("INSERT INTO Products VALUES({:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?})",
        &pid,&pname.trim(),&ptype.trim(),&retailprice,&brand.trim(),&discountprice,&rating.trim(),&status.trim());
        
        conn.query::<String, std::string::String>(qr).unwrap();
        println!("Product Added!!!");
        println!("\npid: {:?}\npname: {:?}\nptype: {:?}\nretailprice: {:?}\nbrand: {:?}\ndiscountprice: {:?}\nrating: {:?}\nstatus: {:?}",
        &pid,&pname.trim(),&ptype.trim(),&retailprice,&brand.trim(),&discountprice,&rating.trim(),&status.trim());

    }
    fn delete_product(){
        let mut conn = init_db();

        let mut pid = String::new();
        println!("Enter the Product id:");
        std::io::stdin().read_line(&mut pid).unwrap();
        let qr = format!("Delete FROM Products WHERE Pid ={:?})",&pid.trim());
        println!("Product id : {:?} Deleted..",&pid.trim());

    }

    fn product_analysis(){

        let mut conn = init_db();

        println!("\n\t\t*** Products Available ***");
        let mut qr = format!("SELECT PType, COUNT(*) AS `num` FROM Products GROUP BY PType;");
        let mut res:Vec::<(String, i64)> = conn.query(qr).unwrap();
        println!("\tProduct Type                | Count ");
        println!("--------------------------------------------");
        for r in res {
            println!("{}    | {} ", r.0, r.1);
        }

        println!("\nSpecify the Product type to analyse..");
        let prod_type = Self::get_product_Type();
        println!("\n\t\t***{:?} Brands Available ***",&prod_type);
        qr = format!("SELECT Brand, COUNT(*) AS `num` FROM productdb.products WHERE PType = {:?} GROUP BY Brand;",&prod_type);
        let res:Vec::<(String, i64)> = conn.query(qr).unwrap();
        println!("Brand Name           | Count ");
        
        println!("--------------------------------------");
        for r in res {
            println!("{}    | {} ", r.0, r.1);
        }
    }
       
}
struct user{
    Cid: String,
    Username: String,
    Password: String
}

impl user{
    fn validate_user(username: &str, password: &str){
        let mut conn = init_db();
        let db_pwd=conn.exec_first::<_, &str, Params>("select Cid,Username,Password from Customers where Username = :username",
            params!{"username"=>&username,}).map(|row|{
                row.map(|(Cid, Username,Password)|user{
                    Cid:Cid, Username:Username,Password:Password,
                })
            }).expect("Query failed");

        let usr = db_pwd.unwrap();
        if password == usr.Password{
            println!("\tLogin Successfull!!\n\n\n\t\t Welcome {:?}",&username);
            loop{
                Product::view_products(&usr.Cid);
                println!("\nDo you want more products <yes/no>: ");
                let mut ch = String::new();
                std::io::stdin().read_line(&mut ch).unwrap();
                let choice = ch.trim().to_lowercase();
                if choice == "no" {
                    println!("\t\tLogout successful!! Thanks for Shopping..");
                    break;
                }         
            }
        }
        else{
            println!("\tLogin Unsuccessfull!! please enter valid username or password");
        }
    }

    

}
enum Access {
    Admin,
    Analyst,
    User,
}

fn main() {
    // Connecting to MYSQL DATABASE
    let conn = init_db();
    println!("Database connected..");

    //Setting the Access levels
    let mut Access_level = Access::User;
    // let mut Access_level = Access::Admin;
    // let mut Access_level = Access::Analyst;

    match Access_level{
        Access::Admin => admin_login(),
        Access::Analyst => analyst_login(),
        Access::User => user_login(),
    };
}
fn admin_login(){

    println!("\n\n\t\t*** Welcome Admin ***");
    println!("\n1.Add Product\n2.Delete Product\n3.View Orders\n");
    println!("\n Enter the option:");
    let mut ch = String::new();
    std::io::stdin().read_line(&mut ch).unwrap();
    match ch.trim().parse().expect("enter valid number"){
        1=> Product::add_product(),
        2=> Product::delete_product(),
        3=> Orders::view_Orders(),
        _=> println!("Incorrect option!! Enter the Valid Option!!!")
    };
}

fn analyst_login(){
    println!("\n\n\t\t*** Welcome Analyst ***");
    println!("\n1.Product Analysis\n2.Customer Analysis");
    println!("\n Enter the option:");
    let mut ch = String::new();
    std::io::stdin().read_line(&mut ch).unwrap();
    match ch.trim().parse().expect("enter valid number"){
        1=> Product::product_analysis(),
        2=> Orders::customer_analysis(),
        _=> println!("Incorrect option!! Enter the Valid Option!!!")
    };

}

fn user_login(){
    println!("\n\t\t**** Login Page ****");
    let mut username = String::new();
    println!("Enter Username :");
    std::io::stdin().read_line(&mut username).unwrap();

    let mut pswrd = String::new();
    println!("Enter password :");
    std::io::stdin().read_line(&mut pswrd).unwrap();
    user::validate_user(&username.trim(),&pswrd.trim());
}
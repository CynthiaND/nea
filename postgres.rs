use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Product {
    product_id: Uuid,
    product_name: String,
    product_image: String,
    product_description: String,
    price: u16,
}
#[derive(Serialize, Deserialize)]
struct User {
    user_id: Uuid,
    password_hash: String,
    salt: String,
    order_ids: Vec<Uuid>,
}
#[derive(Serialize, Deserialize)]
struct Order {
    order_id: Uuid,
    cart: Vec<String>,
    user_id: Uuid,
}

fn client() -> Client {
    let client = Client::connect(
        "host=localhost user=snow pass=rustontop dbname=flake",
        NoTls,
    ).unwrap();

    return client
}

pub fn init_tables() {
    let mut client = client();
    let transaction = client.transaction();
    let mut transaction = transaction.unwrap();
    transaction.batch_execute(
        "
    	CREATE TABLE users (
    		userId UUID PRIMARY KEY NOT NULL,
            email VARCHAR(255) NOT NULL,
    		passwordHash CHAR(16) NOT NULL,
    		salt CHAR(16) NOT NULL,
            order_ids UUID[],
            );
        CREATE TABLE order (
            order_id UUID PRIMARY KEY NOT NULL,
            items TEXT[] NOT NULL,
            user_id UUID REFERENCES users NOT NULL,
            );
        CREATE TABLE product (
            productId UUID PRIMARY KEY NOT NULL,
            productName CHAR(50) NOT NULL,
            productImage CHAR(50) NOT NULL,
            productDescription CHAR(50) NOT NULL,
            price SMALLINT NOT NULL,
            );
        ",
    );
    transaction.commit();
}

//Password Hashing Functions
fn hash_password_existing(password: String, password_salt: String) -> String {
    let hash_string = SaltString::from_b64(&password_salt).unwrap();
    let result = Argon2::default().hash_password(password.as_bytes(), &hash_string).unwrap();
    return result.to_string()
}

fn hash_password(password: &str) -> (String, String) {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
    return (password_hash.to_string(), salt.to_owned().to_string());
}


//JSON functions
fn products() -> Vec<String> {
    let mut client = client();
    let mut products: Vec<Product> = Vec::new();
    let mut json_products: Vec<String> = Vec::new();
    for row in client.query("SELECT productID, productName,productImage, productDescription, price FROM products", &[]).unwrap() {
        let uuid: String = row.get(0);
        let price: String = row.get(4);
        let new_product = Product {
            product_id: Uuid::parse_str(&uuid).unwrap(),
            product_name: row.get(1),
            product_image: row.get(2),
            product_description: row.get(3),
            price: str::parse(&price).unwrap(),
        };
        products.push(new_product);
    }
    for item in products {
        let json_item = serde_json::to_string(&item);
        json_products.push(json_item.unwrap());
    }
    return json_products
}

fn product_price(product_id: Uuid) -> String {
    let mut client = client();
    let mut price: String = "".into();
    for row in client.query("SELECT price FROM products WHERE productId = ${product_id}", &[]).unwrap() {
        price = row.get(0);
    }
    return serde_json::to_string(&price).unwrap();
}

fn product_name(product_id: Uuid) -> String {
    let mut client = client();
    let mut name: String = "".into();
    for row in client.query("SELECT productName FROM products WHERE productId = ${product_id}", &[]).unwrap() {
        name = row.get(0);
    }
    return serde_json::to_string(&name).unwrap();
}

//User Functions
fn user_exists(email: String, password: String) -> bool {
    let mut client = client();
    let mut password_salt: String;
    let mut password_hash: String;
    for user in client.query("SELECT salt, passwordHash FROM users WHERE email = ${email}", &[]).unwrap() {
        if user.is_empty() {
            return false;
        }
        password_salt = user.get(0);
        password_hash = user.get(1);
        if password_salt.is_empty() && password_hash.is_empty() {
            return false;
        }
    if password_hash == hash_password_existing(password.clone(), password_salt) {
        return true;
    }
    }
    return false
}


fn new_user(email: String, password: String) {
    let mut client = client();
    let response = hash_password(&password);
    let user_id = Uuid::new_v4();
    client.execute(
        "INSERT INTO users (userID, email, passwordHash, salt, order_ids) VALUES ($1, $2, $3, $4, $5)",
        &[&user_id.to_string(), &email, &response.0, &response.1, &""],
    ).unwrap();
}
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// =======================
// STRUCT
// =======================

#[contracttype]
#[derive(Clone, Debug)]
pub struct Item {
    id: u64,
    name: String,
    stock: u32,
    price: u32,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Sale {
    item_id: u64,
    quantity: u32,
    total_price: u32,
    timestamp: u64,
}

// =======================
// STORAGE KEY (FIXED)
// =======================

const ITEM_DATA: Symbol = symbol_short!("ITEMS"); // <= 5 char
const SALES_DATA: Symbol = symbol_short!("SALES"); // <= 5 char (FIX)

// =======================

#[contract]
pub struct InventoryContract;

#[contractimpl]
impl InventoryContract {

    // =======================
    // ITEM FUNCTION
    // =======================

    pub fn get_items(env: Env) -> Vec<Item> {
        env.storage().instance().get(&ITEM_DATA).unwrap_or(Vec::new(&env))
    }

    pub fn add_item(env: Env, name: String, stock: u32, price: u32) -> String {
        if stock == 0 {
            return String::from_str(&env, "Stock cannot be zero");
        }

        if price == 0 {
            return String::from_str(&env, "Price cannot be zero");
        }

        let mut items: Vec<Item> = env.storage().instance().get(&ITEM_DATA).unwrap_or(Vec::new(&env));

        let item = Item {
            id: env.prng().gen::<u64>(),
            name,
            stock,
            price,
        };

        items.push_back(item);
        env.storage().instance().set(&ITEM_DATA, &items);

        String::from_str(&env, "Item added successfully")
    }

    pub fn delete_item(env: Env, id: u64) -> String {
        let mut items: Vec<Item> = env.storage().instance().get(&ITEM_DATA).unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            if items.get(i).unwrap().id == id {
                items.remove(i);
                env.storage().instance().set(&ITEM_DATA, &items);
                return String::from_str(&env, "Item deleted");
            }
        }

        String::from_str(&env, "Item not found")
    }

    // =======================
    // SALES FUNCTION
    // =======================

    pub fn sell_item(env: Env, id: u64, qty: u32) -> String {
        if qty == 0 {
            return String::from_str(&env, "Quantity must be greater than 0");
        }

        let mut items: Vec<Item> = env.storage().instance().get(&ITEM_DATA).unwrap_or(Vec::new(&env));
        let mut sales: Vec<Sale> = env.storage().instance().get(&SALES_DATA).unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            let mut item = items.get(i).unwrap();

            if item.id == id {

                if item.stock < qty {
                    return String::from_str(&env, "Stock not enough");
                }

                // update stock
                item.stock -= qty;
                items.set(i, item.clone());

                // create sale
                let sale = Sale {
                    item_id: id,
                    quantity: qty,
                    total_price: qty * item.price,
                    timestamp: env.ledger().timestamp(),
                };

                sales.push_back(sale);

                env.storage().instance().set(&ITEM_DATA, &items);
                env.storage().instance().set(&SALES_DATA, &sales);

                return String::from_str(&env, "Transaction success");
            }
        }

        String::from_str(&env, "Item not found")
    }

    pub fn get_sales(env: Env) -> Vec<Sale> {
        env.storage().instance().get(&SALES_DATA).unwrap_or(Vec::new(&env))
    }

    // =======================
    // ANALYTICS
    // =======================

    pub fn get_total_revenue(env: Env) -> u32 {
        let sales: Vec<Sale> = env.storage().instance().get(&SALES_DATA).unwrap_or(Vec::new(&env));

        let mut total: u32 = 0;

        for i in 0..sales.len() {
            total += sales.get(i).unwrap().total_price;
        }

        total
    }
}

mod test;
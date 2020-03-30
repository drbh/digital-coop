use juniper::FieldResult;
use juniper::RootNode;

use hotpot_db::*;
use serde::{Deserialize, Serialize};

use nanoid::nanoid;

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLObject, Serialize, Deserialize)]
#[graphql(description = "A SKU item")]
struct Item {
    id: String,
    name: String,
    sku: String,
    unit: String,
    organic: String,
    brand: String,
    price: f64,
}

#[derive(GraphQLObject, Serialize, Deserialize)]
#[graphql(description = "A Order item")]
struct OrderItem {
    id: String,
    item: Item,
    price: f64,
    quantity: f64,
}

#[derive(GraphQLObject, Serialize, Deserialize)]
#[graphql(description = "A Order")]
struct Order {
    id: String,
    member_id: String,
    items: Vec<OrderItem>,
    price: f64,
    quantity: f64,
}

#[derive(GraphQLObject, Serialize, Deserialize)]
#[graphql(description = "A timeslot")]
struct Timeslot {
    id: String,
    time: String,
    orders_id: Vec<String>,
}

//
// NEW IEMS
//

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A SKU item")]
struct NewItem {
    name: String,
    sku: String,
    unit: String,
    organic: String,
    brand: String,
    price: f64,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "An order and item to remove")]
struct RemoveItem {
    order_id: String,
    item_id: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A Order item")]
struct NewOrderItem {
    order_id: String,
    item_id: String,
    price: f64,
    quantity: f64,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A Order")]
struct NewOrder {
    items: Vec<NewOrderItem>,
    member_id: String,
    price: f64,
    quantity: f64,
}


#[derive(GraphQLInputObject)]
#[graphql(description = "A new Timeslot")]
struct NewTimeslot {
    time: String,
    orders_id: Vec<String>,
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn human(id: String) -> FieldResult<Human> {
        Ok(Human {
            id: nanoid!().to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }
    fn item(id: String) -> FieldResult<Item> {
        let mut pot = HotPot::new("."); // handle connection closing

        let query = QueryBuilder::new()
            .collection("items")
            .kind(QueryKind::Object)
            .key("id")
            .comparison("=")
            .string(&id)
            .finish();

        let results = pot.execute(query);
        let first_res = results.unwrap();
        let second_res = first_res.first().unwrap();
        let my_item: Item = serde_json::from_str(&second_res.data).unwrap();
        Ok(my_item)
    }
    fn items() -> FieldResult<Vec<Item>> {
        let mut pot = HotPot::new("."); // handle connection closing

        let query = QueryBuilder::new()
            .collection("items")
            .kind(QueryKind::Object)
            .key("id")
            .comparison(">")
            .string("0")
            .finish();

        let results = pot.execute(query);
        let first_res = results.unwrap();
        let mut items: Vec<Item> = Vec::new();
        for second_res in first_res {
            let my_item: Item = serde_json::from_str(&second_res.data).unwrap();
            items.push(my_item)
        }
        Ok(items)
    }
    fn timeslots() -> FieldResult<Vec<Timeslot>> {
        let mut pot = HotPot::new("."); // handle connection closing

        let query = QueryBuilder::new()
            .collection("timeslots")
            .kind(QueryKind::Object)
            .key("id")
            .comparison(">")
            .string("0")
            .finish();

        let results = pot.execute(query);
        let first_res = results.unwrap();
        let mut timeslots: Vec<Timeslot> = Vec::new();
        for second_res in first_res {
            let my_slot: Timeslot = serde_json::from_str(&second_res.data).unwrap();
            timeslots.push(my_slot)
        }
        Ok(timeslots)
    }
    fn order(id: String) -> FieldResult<Order> {
        let mut pot = HotPot::new("."); // handle connection closing

        println!("\nLOOKING FOR {:?}", id);
        let query = QueryBuilder::new()
            .collection("orders")
            .kind(QueryKind::Object)
            .key("id")
            .comparison("=")
            .string(&id)
            .finish();

        let results = pot.execute(query);
        let first_res = results.unwrap();
        let second_res = first_res.first().unwrap();
        let my_order: Order = serde_json::from_str(&second_res.data).unwrap();
        Ok(my_order)
    }
    fn orders() -> FieldResult<Vec<Order>> {
        let mut pot = HotPot::new("."); // handle connection closing

        let query = QueryBuilder::new()
            .collection("orders")
            .kind(QueryKind::Object)
            .key("id")
            .comparison(">")
            .string("0")
            .finish();

        let results = pot.execute(query);
        let first_res = results.unwrap();
        let mut orders: Vec<Order> = Vec::new();
        for second_res in first_res {
            let my_item: Order = serde_json::from_str(&second_res.data).unwrap();
            orders.push(my_item)
        }
        Ok(orders)
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
    fn createHuman(new_human: NewHuman) -> FieldResult<Human> {
        let mut pot = HotPot::new("."); // handle connection closing

        Ok(Human {
            id: nanoid!().to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
    fn createTimeslot(new_timeslot: NewTimeslot) -> FieldResult<Timeslot> {
        let mut pot = HotPot::new("."); // handle connection closing
        let timeslot = Timeslot{
            id: nanoid!().to_owned(),
            time: new_timeslot.time, // "Mar 26th 11:00 AM".to_string(),
            orders_id: new_timeslot.orders_id// vec!["0xABCD".to_string(), "0xPOIU".to_string()],
        };
        let r = pot.insert::<Timeslot>("timeslots", &timeslot).unwrap();
        Ok(timeslot)
    }
    fn createItem(new_item: NewItem) -> FieldResult<Item> {
        let mut pot = HotPot::new("."); // handle connection closing

        let uid = nanoid!();
        println!("{:?}", uid);
        // well make a new item we want to store
        let person = Item {
            id: uid.to_string(),
            name: new_item.name.to_string(),
            sku: new_item.sku.to_string(),
            unit: new_item.unit.to_string(),
            organic: new_item.organic.to_string(),
            brand: new_item.brand.to_string(),
            price: new_item.price,
        };

        // we insert the object into the collection!
        let r = pot.insert::<Item>("items", &person).unwrap();
        println!("{:?}", r);

        Ok(person)
    }
    fn createOrder(new_order: NewOrder) -> FieldResult<Order> {
        let mut pot = HotPot::new("."); // handle connection closing

        let uid = nanoid!();
        let ord = Order {
            id: uid.to_string(),
            member_id: new_order.member_id,
            items: vec![], //new_order.items,
            price: new_order.price,
            quantity: new_order.quantity,
        };
        let r = pot.insert::<Order>("orders", &ord).unwrap();
        // println!("{:?}", r);

        Ok(ord)
    }
    fn addItemToOrder(new_order_item: NewOrderItem) -> FieldResult<Order> {
        // println!("{:?}", new_order_item);

        let mut pot = HotPot::new("."); // handle connection closing

        let query = QueryBuilder::new()
            .collection("items")
            .kind(QueryKind::Object)
            .key("id")
            .comparison("=")
            .string(&new_order_item.item_id)
            .finish();

        let results = pot.execute(query);
        let first_res = results.unwrap();
        let second_res = first_res.first().unwrap();
        let my_item: Item = serde_json::from_str(&second_res.data).unwrap();

        let query = QueryBuilder::new()
            .collection("orders")
            .kind(QueryKind::Object)
            .key("id")
            .comparison("=")
            .string(&new_order_item.order_id)
            .finish();

        let results = pot.execute(query);
        let first_res = results.unwrap();
        let second_res = first_res.first().unwrap();
        let mut my_order: Order = serde_json::from_str(&second_res.data).unwrap();

        let uid = nanoid!();
        let oi = OrderItem {
            id: uid.to_string(),
            item: my_item,
            price: new_order_item.price,
            quantity: new_order_item.quantity,
        };

        my_order.items.push(oi);

        // println!("overwrite at {:?}", second_res.id);
        let r = pot
            .upsert_at_index::<Order>("orders", second_res.id as usize, &my_order)
            .unwrap();
        println!("{:?}", r);

        Ok(my_order)
    }
    fn removeItemFromOrder(remove_item: RemoveItem) -> FieldResult<Order> {
        let mut pot = HotPot::new("."); // handle connection closing
        let query = QueryBuilder::new()
            .collection("orders")
            .kind(QueryKind::Object)
            .key("id")
            .comparison("=")
            .string(&remove_item.order_id)
            .finish();

        let results = pot.execute(query);
        let first_res = results.unwrap();
        let second_res = first_res.first().unwrap();
        let mut my_order: Order = serde_json::from_str(&second_res.data).unwrap();

        Ok(my_order)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {

    let mut pot = HotPot::new("."); // handle connection closing
    pot.create_collection("items");
    pot.create_collection("orders");
    pot.create_collection("timeslots");

    Schema::new(QueryRoot {}, MutationRoot {})
}


```json
[
    [
        "Apple-all other IPM",
        "per pound",
        "$1.32",
        "Integrated Pest Management",
        "Locally grown within 500 miles"
    ]
]
```


## Orders

```graphql
mutation {
  createOrder(newOrder: {
    	items: [],
    	price: 0.0,
    	quantity: 0.0,
  	}) {
      id
      items {
        id
        item {
          id
          name
          sku
        }
        price
        quantity
      }
      price
      quantity
  }
}
```

```graphql
{
  orders {
    id
    items {
      id
      item {
        sku
        name
      }
      price
      quantity
    }
    price
    quantity
  }
}
```

```
{
  order(id: "C-uhb5mfPkQYxO1eLxBJf") {
    id
    items {
      id
      item {
        sku
        name
      }
      price
      quantity
    }
    price
    quantity
  }
}
```

```
mutation {
  addItemToOrder(newOrderItem: {
    orderId: "C-uhb5mfPkQYxO1eLxBJf", 
    itemId: "sEfo_DCEJr2CA95U9bsIW", 
    price: 100.0, 
    quantity: 0.0
  }) {
    id
    items {
      id
      item {
        id
        name
        sku
      }
    }
    price
    quantity
  }
}
```


## Items

```graphql
{
  item(id: "sEfo_DCEJr2CA95U9bsIW") {
    id
    name
    sku
  }
}
```

```graphql
mutation {
  createItem(newItem: {
      name: "Pizza", 
      sku: "0X99090"
    }) {
    name
    sku
  }
}
```


```graphql
{
  items {
    id
    name
    sku
  }
}
```

## Timeslots

```graphql
mutation {
  createTimeslot(newTimeslot: {
      time: "Fri Mar 27 2020 01:00:00 GMT-0400",
      ordersId: ["ABCD", "XYZA"],
    }) {
      id
      time
      ordersId
  }
}
```

```
{
  timeslots {
    time
    ordersId
  }
}
```

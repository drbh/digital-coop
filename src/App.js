import React, { Fragment, useEffect, useState } from "react";
import "./App.css";

// function range(start, stop, step) {
//   var a = [start],
//     b = start;
//   while (b < stop) {
//     a.push((b += step || 1));
//   }
//   return a;
// }

function graphqlQuery(body, callback) {
  let cleanBody = `{ "query": "${body}" }`
    .replace(/\s/g, " ")
    .replace(/\s+/g, " ")
    .trim();
  console.log(cleanBody);
  var fullUrl = window.location.hostname; //+ window.location.pathname;
  let url = "http://" + fullUrl + ":8080/graphql";
  fetch(url, {
    method: "POST",
    headers: {
      "content-type": "application/json",
    },
    body: cleanBody,
  })
    .then((response) => {
      response.json().then((data) => {
        console.log(data);
        callback(data.data);
      });
    })
    .catch((err) => {
      console.log(err);
    });
}

function ItemList(props) {
  const [localItems, setLocalItems] = useState([]);

  function getQuantity(id, cart) {
    let va = 0;
    if (id in cart) {
      va = cart[id];
    }
    return va;
  }

  function searchFood(event) {
    var filtered = props.items.filter((value) => {
      return value.name
        .toLowerCase()
        .includes(event.target.value.toLowerCase());
    });
    setLocalItems(filtered);
  }

  useEffect(() => {
    setLocalItems(props.items);
  }, []);

  return (
    <div className="items">
      <div className="items-header">
        <div className="h2-like">🥦 Produce 🥕</div>
        <small>{localItems.length} items</small>
      </div>
      {true && (
        <div className="items-search">
          <input
            className="items-search-input"
            type="text"
            placeholder="  Find a food..."
            onChange={searchFood}
          />
        </div>
      )}
      <div className="card-container">
        {localItems.map((data, index) => {
          return (
            <div key={index} className="card" itemdid={data.id}>
              <div className="card-head">
                <span className="product-name">{data.name}</span>-
                <span className="product-brand">{data.brand}</span>
              </div>
              <br />
              <span className="product-price">
                ${data.price} {data.unit}
              </span>
              <br />
              <span className="product-organic">{data.organic}</span>
              <br />
              <div className="cart-quantity">
                <label htmlFor="quantity">Quantity:</label>

                <select
                  id="defaultNumbers"
                  value={getQuantity(data.id, props.cartCounts)}
                  onChange={(e) => {
                    //
                    props.updateCart(data.id, e.target.value);
                  }}
                >
                  {[...Array(10).keys()].map((i) => {
                    return <option key={i} value={i}>{i}</option>;
                  })}
                </select>

                {/*                <input
                  type="number"
                  pattern="\d*"
                  id="quantity"
                  name="quantity"
                  min="0"
                  max="99"
                  value={getQuantity(data.id, props.cartCounts)}
                  onClick={(e)=>{
                    // e.target.focus();
                    e.target.setSelectionRange(0, e.target.value.length)
                  }}
                  onChange={(e) => {
                    //
                    props.updateCart(data.id, e.target.value);
                  }}
                />*/}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
function CurrentOrder(props) {
  const [subTotal, setSubTotal] = useState(0);

  const [timeslot, setTimeslot] = useState(0);

  const [isActive, setIsActive] = useState(true);

  function getTotalPrice() {
    var cartSubtotal = 0;
    for (let key of Object.keys(props.currentCart)) {
      let quantity = props.currentCart[key];
      let product = props.items.filter((entry) => {
        return entry.id === key;
      })[0];
      // console.log(product);
      let productSubtotal = product.price * quantity;
      cartSubtotal += productSubtotal;
    }
    setSubTotal(cartSubtotal);
    // console.log(props.currentCart)
  }
  useEffect(() => {
    getTotalPrice();
  }, [props.currentCart]);
  return (
    <div className={isActive ? "order is-active" : "order"}>
      <h4
        onClick={() => {
          setIsActive(!isActive);
        }}
      >
        My order: ${subTotal}
      </h4>

      <br />

      <div className="order-panel">
        <h3>Review Order</h3>

        <div className="subtotal-list">
          <table>
            <thead>
            <tr>
              <th>Name</th>
              <th>Quantity</th>
              <th>Subtotal</th>
            </tr>
            </thead>
            <tbody>
            {Object.keys(props.cart).map((key, index) => {
              var filtered = props.items.filter((value) => {
                return value.id === key;
              })[0];
              var quant = props.cart[key];
              var subTotal = filtered.price * quant;
              {
                /*console.log(filtered)*/
              }
              return (
                <tr key={index}>
                  <td>{filtered.name} </td>
                  <td>{quant} </td>
                  <td>{subTotal}</td>
                </tr>
              );
            })}

            <tr>
              <td>{"Total"} </td>
              <td> </td>
              <td>{subTotal}</td>
            </tr>
            </tbody>
          </table>
        </div>

        {
          false &&
          <Fragment>
          <br />
          <h3>Timeslots</h3>

          <div className="slot-chooser">
            <h4>March 26th</h4>

            <div className="timeslot-list">
              {[7, 8, 9, 10].map((v) => {
                return (
                  <div key={v} className={"timeslot " + (true ? "is-active" : "is-active")}>
                    <div className="timeslot-time">{v}:00 AM</div>
                    <div className="timeslot-counter">{v}</div>
                  </div>
                );
              })}
            </div>
          </div>
          </Fragment>
        }
        {/*</div>*/}
        <input
          type="button"
          value="Confirm"
          onClick={() => {
            //
            console.log(timeslot)
            console.log(props.cart)
          }}
        />
      </div>
    </div>
  );
}

function App() {
  const [userState, setUserState] = useState("shop");

  const [cart, setCart] = useState({});
  const [memberId, setMemberId] = useState("#1234567");
  const [items, setItem] = useState(null);
  const [order, setOrder] = useState(null);

  function populateItems() {
    let query = `{  
      items {    
        id
        name
        sku
        unit
        organic
        brand
        price
      }
    }`;
    graphqlQuery(query, setItem);
  }

  function populateOrder(orderId) {
    let query = `{
      order(id: \\"5OLXrjSEzsdpfDiERe9em\\") {
        id
        memberId
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
    }`;
    graphqlQuery(query, setOrder);
  }

  function startNewOrder() {
    let query = `mutation {
      createOrder(newOrder: {items: [], memberId: "dvid", price: 0.0, quantity: 0.0}) {
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
    }`;
    graphqlQuery(query, console.log);
  }

  function addItemToOrder(orderId, itemId) {
    let query = `mutation {
        addItemToOrder(newOrderItem: {
          orderId: \\"${orderId}\\", 
          itemId: \\"${itemId}\\", 
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
      }`;
    graphqlQuery(query, console.log);
  }

  useEffect(() => {
    populateItems();
    // startNewOrder()
    // populateOrder("5OLXrjSEzsdpfDiERe9em");
    // addItemToOrder("5OLXrjSEzsdpfDiERe9em", "gqrUvdVK1yAL4I4-PRD5o")
  }, []);

  // renderSwitch(param) {
  //   switch(param) {
  //     case 'foo':
  //       return 'bar';
  //     default:
  //       return 'foo';
  //   }
  // }

  return (
    <div className="app">
      <div className="nav">
        <div className="member-header">{memberId}</div>
      </div>
      <div className="spacer"></div>
      <div className="container">
        {userState === "init" && (
          <div className="center-stage">
            <h2>Member ID</h2>
            <input type="text" />
            <input
              type="button"
              value="shop"
              onClick={() => {
                setUserState("shop");
              }}
            />
          </div>
        )}

        {userState === "shop" && (
          <div className="shop-container">
            {items && (
              <div>
                <ItemList
                  items={items.items}
                  cartCounts={cart}
                  updateCart={(id, quantity) => {
                    cart[id] = quantity;
                    if (quantity === 0) {
                      delete cart[id];
                    }
                    setCart({
                      ...cart,
                    });
                  }}
                />
                <CurrentOrder
                  cart={cart}
                  items={items.items}
                  orders={order}
                  currentCart={cart}
                />
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}

export default App;

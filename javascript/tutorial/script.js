// let, const

let age = 30; // Mutable variable
age += 1;
console.log(age);

const ID = 0123456789;  // Imutable variable
console.log(ID);

// String, Number (ew), Boolean, null, undefined, Symbol

let name = "John";      // String
//let age = 30;           // Number
let rating = 3.0;       // Number
const isCool = true;    // Boolean
const x = null;         // null
const y = undefined;    // undefined
let z;                  // undefined


console.log("What idiot thought auto-concatination was a good idea " + 3);
console.log(`This is better ${name} ${age} ${ID}`);

const s = "Javascript isn\'t an amazing language...";
console.log(s.length);
console.log(s.toUpperCase());
console.log(s.toLowerCase());
// String slicing
console.log(s.substring(0, 13) + s.substring(16, s.length));
// // String -> Array
// console.log(s.split('')); // Commented out to prevent log junk

/*
const weird_numbers = new Array(1,2,3,4,5,6);
console.log(weird_numbers);
*/

const fruits = ["apples", "oranges", "tomatoes"];
console.log(fruits);

fruits[3] = "grapes"; // WHYYYYYYYYYYYYYYYY
fruits.push("Pear");  // So much for it being constant -_-
const pairs = fruits.pop();

console.log(fruits[0]);



const gross_dynamic_typed_array = ["Javascript", 100, false];
console.log(`${gross_dynamic_typed_array[0]} being good is ${gross_dynamic_typed_array[1]}% ${gross_dynamic_typed_array[2]}`);

const person = {
  firstName: "John",
  lastName: "Doe",
  age: age,
  opinion: "Javascript is... not good",
  address: {
    street: ""
  }
}

const jsonified = JSON.stringify(person);


for (let x=0; x<30; x++) {
  let y = 0;
  while (y<x) {
    dummy = y < 15 ? "low" : "high";
    y++;
  }
}


for (let i=0; i<fruits.length; i++) {
  console.log(fruits[i])
}

for (let fruit of fruits) {
  console.log(fruit)
}

fruits.forEach(function(fruit) {
  console.log(fruit); // good Javascript
})

const yummy_fruits = fruits.map(function(fruit) {
  fruit += " yum";
})

if ("10" == 10) {
  console.log("bad Javascript");
}
if ("10" === "10") {
  // why
} else if ("10" === "20") {

} else if (true || false && isCool) {

} else {
  const colour = 1 > 2 ? "Red" : "Blue";

  switch(colour) {
    case "Red":
      console.log("Javascript switch cases are stupid too.");
      break;
    case "Blue":
      console.log("Be consistent???");
      break;
    default:
      console.log("colour can not make purple");
      break;
  }
}

function add(x,y) {
  console.log(x,y);
  return x+y;
}

add();
add(1,2);

function add2(x=add(2,1), y=add(x,2)) {
  console.log("This shouldn't work");
  return add(x,y);
}

const add_nums = (x, y) => {
  return x + y;
}



const add5 = x => x + 5;

function Train() {
  this.song = "This dot, This dot";
  return this;
}

const train = Train();
console.log(train.song);

function Cart() {
  this.song = train.song;
  this.sing = () => {console.log(this.song)};
}

const cart = new Cart();
console.log(cart.song);
cart.sing();

class Other_Train {
  constructor(name, song="This dot, This dot") {
    this.name=name;
    this.song=song;
  }
  sing() { //Javascript is bad
    console.log(this.song);
  }
}

const other_train = new Other_Train("joe");
other_train.sing();


// === === === === === === === === === === === === ===

console.log(window);
window.alert(fruits);

const header = document.getElementByID("my_header");
console.log(header);

header.innerText = ";-; This language is bad";
header.innerHTML = "<p>Hello....</p>";

const btn = document.querySelector(".btn");
btn.addEventListener("click", (c) => {
  c.preventDefault();
  console.log("CLICKED");
  btn.innerText = "clicked...";
})






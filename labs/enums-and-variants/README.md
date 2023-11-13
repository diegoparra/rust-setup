# Enums and Variants


## Enum Shape

- Create an Enum `Shape` that has two fields of type f64:
  - Circle
  - Square

- Into Main function create a variable `shapes` of vec containing both Enums of `Shape`

- Create a varible `total_area` from `shapes`
  - use the function `iter` to iterate over it.
  - use the function `map` to `match` over `shape`
    - map(|shape| match shape {
          Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
          Shape::Square(lenght) => lenght * lenght
      })
  - use the function `sum` to get the total area result

- Print the total area calling the variable `total_area`


## 
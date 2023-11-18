# enum 和 trait object 

## 背景

`vector`只能存储相同类型的值，这是很不方便的，一定会需要存储一系列不同类型的值的用例，解决的方法有两种：

1. `enum`枚举的变体都被定义为相同的枚举类型，当需要在`vector`中存储不同类型值的时候，可以定义并使用一个枚举；
2. 定义一个存放`trait object`的`vector`，`trait object`指向一个实现了指定`trait`的类型的实例。

## 使用 enum

- 首先定义不同的类型，并未不同的类型实现各自的方法

  ```rust
  struct Cat {}
  impl Cat {
      fn talk(&self) {
          println!("meow");
      }
  }
  
  struct Dog {}
  impl Dog {
      fn talk(&self) {
          println!("bark");
      }
  }
  
  struct Sheep {}
  impl Sheep {
      fn talk(&self) {
          println!("baa");
      }
  }
  ```

- 定义枚举类型，将不同的类型包裹进不同的变体中

  ```rust
  enum Animal {
      Cat(Cat),
      Dog(Dog),
      Sheep(Sheep),
  }
  ```

- 实例化一个枚举，对实例使用`for`循环，通过使用`match`来调用不同变体的方法

  ```rust
  fn main() {
      let animals = vec![
          Animal::Cat(Cat {}),
          Animal::Dog(Dog {}),
          Animal::Sheep(Sheep {}),
      ];
  
      for animal in &animals {
          match animal {
              Animal::Cat(c) => c.talk(),
              Animal::Dog(d) => d.talk(),
              Animal::Sheep(s) => s.talk(),
          }
      }
  }
  ```

## 使用 trait object

- 定义`trait`特征

  ```rust
  trait Animal {
      fn talk(&self) {}
  }
  ```

- 定义每一个类型，并为每个类型实现`trait`

  ```rust
  struct Cat {}
  impl Animal for Cat {
      fn talk(&self) {
          println!("meow");
      }
  }
  
  struct Dog {}
  impl Animal for Dog {
      fn talk(&self) {
          println!("bark");
      }
  }
  
  struct Sheep {}
  impl Animal for Sheep {
      fn talk(&self) {
          println!("baa");
      }
  }
  ```

- 实例化各个类型，利用`trait object`将各个类型实例的例子存储到`vector`中。

  ```rust
  fn main() {
      let c = Cat {};
      let d = Dog {};
      let s = Sheep {};
  
      let animals: Vec<Box<dyn Animal>> = vec![Box::new(c), Box::new(d), Box::new(s)];
  
      for animal in &animals {
          animal.talk();
      }
  }
  ```

## 区别

使用`enum`和`trait object`是在`vector`中存储不同类型的两个方法，这个方法存在区别在：

1. 主要区别在于扩展性不同，`enum`中存储的变体是固定的，当定义好了以后，范围也就固定，使用`trait object`定义了`trait`标准，只要实现`trait`就能利用`trait object`来指定新添加的类型，因此拓展性更好。
2. 在进行`for`循环来调用`vector`中不同类型的方法的时候，`enum`需要匹配各种变体，并为每一中变体实现方法的调用，使用`trait`，由于实现了统一的方法，可以进行快速调用。
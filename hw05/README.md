# HW5: An Adventure Gaem

## My Explaination for `Rc` and `RefCell`

`Rc` provides share ownership which allow us to reference to the same object in the multiple places without worrying about lifetime.
`RefCell` provides dynamic borrow checking which make mutating an object easier when its ownership is shared among multiple places.
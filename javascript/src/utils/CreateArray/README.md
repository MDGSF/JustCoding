# CreateArray

```
@brief CreateArray create arbitrary dimension array.
@params arbitrary numbers of parameters.
    one parameter: create one dimension array.
    two parameter: create two dimension array.
    ....
```

### Example 1

```
const CreateArray = require('@mdgsf/createarray');
const a = CreateArray();
console.log('a =', a); // a = []
```

### Example 2

```
const CreateArray = require('@mdgsf/createarray');
const a = CreateArray(3);
console.log('a =', a); // a = [ null, null, null ]
```

### Example 3

```
const CreateArray = require('@mdgsf/createarray');
const a = CreateArray(2, 3);
console.log('a =', a);
```

output:

```
a = [ [ null, null, null ], [ null, null, null ] ]
```

### Example 4

```
const CreateArray = require('@mdgsf/createarray');
const a = CreateArray(3, 4);
console.log(a);
```

output:

```
[
  [ null, null, null, null ],
  [ null, null, null, null ],
  [ null, null, null, null ]
]
```

### Example 5

```
const CreateArray = require('@mdgsf/createarray');
const a = CreateArray(2, 3, 4);
console.log(a);
```

output:

```
[
  [
    [ null, null, null, null ],
    [ null, null, null, null ],
    [ null, null, null, null ]
  ],
  [
    [ null, null, null, null ],
    [ null, null, null, null ],
    [ null, null, null, null ]
  ]
]
```

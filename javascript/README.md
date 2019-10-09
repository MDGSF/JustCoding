# javascript algorithms

https://github.com/trekhleb/javascript-algorithms

## Install

```
npm install
```

## test

```
npm run test
```

## lint

```
npm run lint
```

## Debug

Open up Chrome and type in the address bar : `chrome://inspect` or `about:inspect`

### debug code

```
node --inspect-brk=9229 app.js
```

### debug test code

```
node --inspect-brk ./node_modules/.bin/jest -i ./src/data-structures/list/__test__/List.test.js
```


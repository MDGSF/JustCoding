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

```
node ./node_modules/jest/bin/jest.js PriorityQueue.test.js
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

## babel

https://www.babeljs.cn/

```
./node_modules/.bin/babel src --out-dir lib
```

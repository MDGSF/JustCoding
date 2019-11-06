/*
 * @brief CreateArray create arbitrary dimension array.
 * @params arbitrary numbers of parameters.
 *     one parameter: create one dimension array.
 *     two parameter: create two dimension array.
 *     ...
 * @example1
 *   const a = CreateArray();
 *   console.log('a =', a); // a = []
 *
 * @example2
 *   const a = CreateArray(3);
 *   console.log('a =', a); // a = [ null, null, null ]
 *
 * @example3
 *   const a = CreateArray(2, 3);
 *   console.log('a =', a); // a = [ [ null, null, null ], [ null, null, null ] ]
 *
 */
function CreateArray() {
  const args = [...arguments];

  if (args.length === 0) {
    return [];
  }

  const arrayLen = args[0];
  const array = new Array(arrayLen).fill(null);
  if (args.length === 1) {
    return array;
  }

  for (let i = 0; i < arrayLen; i += 1) {
    array[i] = CreateArray(...args.slice(1, args.length));
  }

  return array;
}

module.exports = CreateArray;

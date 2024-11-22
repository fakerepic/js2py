function insertionSort(arr) {
    let i = 1;
    while (i < arr.length) {
        const key = arr[i];
        let j = i - 1;
        while (j >= 0 && arr[j] > key) {
            arr[j + 1] = arr[j];
            j = j - 1;
        }
        arr[j + 1] = key;
        i = i + 1;
    }
    return arr;
}
let array = [5.3, 2.1, 8.7, 1.9, 3.4];
console.log(insertionSort(array));

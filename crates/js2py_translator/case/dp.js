function getMaxMatrix(matrix) {
    var m = matrix.length; // 矩阵行数
    var n = matrix[0].length; // 矩阵列数
    var max = matrix[0][0]; // 全局最大值
    var res = [0, 0, 0, 0]; // 最终结果 [r1, c1, r2, c2]

    // 构造列的前缀和
    var preSum = [];
    var i = 0;
    while (i <= m) {
        var row = [];
        var j = 0;
        while (j < n) {
            row.push(0);
            j = j + 1;
        }
        preSum.push(row);
        i = i + 1;
    }

    i = 1;
    while (i <= m) {
        var j = 0;
        while (j < n) {
            preSum[i][j] = preSum[i - 1][j] + matrix[i - 1][j];
            j = j + 1;
        }
        i = i + 1;
    }

    // 合并行
    var top = 0;
    while (top < m) {
        var bottom = top;
        while (bottom < m) {
            // 构造一维矩阵
            var arr = [];
            var k = 0;
            while (k < n) {
                arr.push(preSum[bottom + 1][k] - preSum[top][k]);
                k = k + 1;
            }

            // 最大子数组问题
            var start = 0;
            var sum = arr[0];
            var l = 1;
            while (l < n) {
                if (sum > 0) {
                    sum += arr[l];
                } else {
                    sum = arr[l];
                    start = l;
                }
                if (sum > max) {
                    max = sum;
                    res[0] = top;
                    res[1] = start;
                    res[2] = bottom;
                    res[3] = l;
                }
                l = l + 1;
            }
            bottom = bottom + 1;
        }
        top = top + 1;
    }

    return res;
}

// 测试用例
var matrix = [
    [1, -2, 0],
    [-3, 4, 5],
    [2, -1, 3]
];

console.log(getMaxMatrix(matrix)); 

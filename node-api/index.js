export const handler = async(event) => {
    const querystring = event.queryStringParameters;
    const n = querystring.n || 0;
    const count = counter(n);

    return {
        statusCode: 200,
        body: `Final count is ${count}`,
    };
};

const counter = (n) => {
    let count = 0;
    if (n > 500000000) {
        n = 5000000000;
    }

    for (let i = 0; i <= n; i++) {
        count += i;
    }

    return count;
};
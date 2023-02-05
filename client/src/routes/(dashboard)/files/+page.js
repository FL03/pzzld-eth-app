/** @type {import('./$types').PageLoad} */
export async function load() {
    return {};
};

import path from 'path';


const staticPath = path.resolve('./static');

/* The POST request */
export async function post({ body }) {
    let file;
    let reader = new FileReader();
    reader.readAsDataURL(body);
    reader.onload = e => {
        file = e.target.result
    };
    /* Returns the result */
    return {
        status: 200,
        headers: {
            'content-type': 'text/plain',
        },
        body: 'done',
    };
}
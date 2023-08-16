import axios, { AxiosResponse } from "axios";

const httpClient = axios.create();

// Add request interceptor for extra-logging
httpClient.interceptors.request.use(
    (config) => {
        console.log('Request:', config.method?.toUpperCase(), config.url);
        console.log('Request Data:', config.data);
        return config;
    },
    (error) => {
        console.error('Request Error:', error);
        return Promise.reject(error);
    }
);

// Add response iterceptors for extra-logging
httpClient.interceptors.response.use(
    (response) => {
        console.log('Response:', response.status, response.config.method?.toUpperCase(), response.config.url);
        console.log('Response Data:', response.data);
        return response;
    },
    (error) => {
        console.error('Response Error:', error);
        return Promise.reject(error);
    }
);

function prepareUrl(url: string): string {
    if (url.startsWith("http")) {
        return url;
    } else {
        return "https://" + url;
    }
}

export async function sendRequest(url: string, method: string, requestBody: string, bearerAuth: string): Promise<{ statusCode: number, data: string }> {
    const preparedUrl = prepareUrl(url);

    try {
        const response = await httpClient.request({
            url: preparedUrl,
            method,
            headers: {
                Authorization: `Bearer ${bearerAuth}`
            },
            responseType: 'text',
            data: requestBody,
        });

        return {
            statusCode: response.status,
            data: response.data,
        };
    } catch (err) {
        return {
            statusCode: 0,
            data: `Failed with ${err}`
        };
    }
}

import axios, { AxiosResponse } from "axios";
import { Extension } from "./forms";

const baseUrl = "https://registry.pgtrunk.io/";

const api = axios.create({
    baseURL: baseUrl,
});

// Add request interceptor for extra-logging
api.interceptors.request.use(
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
api.interceptors.response.use(
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

export async function allExtensions(): Promise<Extension[]> {
    try {
        const response: AxiosResponse<Extension[]> = await api.get('/extensions/all');
        return response.data;
    } catch (error) {
        throw new Error(`Failed to fetch orders: ${error}.`);
    }
}

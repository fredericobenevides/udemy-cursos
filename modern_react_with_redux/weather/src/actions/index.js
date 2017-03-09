import axios from 'axios';

const API_KEY = '3e4ba2b42c9f7efed5ecab9e366a2e68';

export const FETCH_WEATHER = 'FETCH_WEATHER';

export const ROOT_URL = `http://api.openweathermap.org/data/2.5/forecast?appid=${API_KEY}`;

export function fetchWeather(city) {
  const url = `${ROOT_URL}&q=${city},us`
  const request = axios.get(url);

  console.log('Request:', request);

  return {
    type: FETCH_WEATHER,
    payload: request
  };
}

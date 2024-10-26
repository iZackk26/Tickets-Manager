import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios';
import ROUTES from '../constants/routes';

class AxiosService {
  private static instance: AxiosService;
  private axiosInstance: AxiosInstance;

  private constructor(baseURL: string) {
    this.axiosInstance = axios.create({
      baseURL,
    });
  }

  public static getInstance(): AxiosService {
    if (!AxiosService.instance) {
      AxiosService.instance = new AxiosService(ROUTES.baseURL);
    }
    return AxiosService.instance;
  }

  async get<T>(url: string, config?: AxiosRequestConfig): Promise<AxiosResponse<T>> {
    try {
      console.log(url)
      const response = await this.axiosInstance.get<T>(url, config);
      return response;
    } catch (error) {
      this.handleError(error);
      throw error;
    }
  }

  async post<T>(url: string, data: any, config?: AxiosRequestConfig): Promise<AxiosResponse<T>> {
    try {
      const response = await this.axiosInstance.post<T>(url, data, config);
      return response;
    } catch (error) {
      this.handleError(error);
      throw error;
    }
  }


  private handleError(error: any) {
    console.error('Error en la solicitud', error);
  }
}

export default AxiosService;

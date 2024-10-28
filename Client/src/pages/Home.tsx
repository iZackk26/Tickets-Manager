import AxiosService from '../classes/AxiosService';
import ROUTES from '../constants/routes';
import Stadium from '../components/Stadium';
import BaseComponent from '../classes/BaseComponent';

class Home extends BaseComponent {
  private apiService: AxiosService;
  private availableZoneSeats: number[];

  constructor(props: {}) {
    super(props);
    this.apiService = AxiosService.getInstance();
    this.availableZoneSeats = [];
  }

  fetchData = async () => {
    try {
      const response = await this.apiService.get<number[]>(ROUTES.availableSeats);
      this.availableZoneSeats = response.data;
      console.log(this.availableZoneSeats);
    } catch (error) {
      console.error('Error al obtener datos:', error);
    }
  };

  render() {
    return (
      <BaseComponent onMount={this.fetchData}>
        <div className="flex flex-row items-center justify-center w-screen h-screen">
          <div className="flex items-center justify-center w-1/2 h-full">
            <Stadium />
          </div>
          <div className="flex flex-col items-center justify-center w-1/2 h-full">
            <h1 className="font-bold text-3xl">
              Welcome to Ticket Manager
            </h1>
            <p className="text-lg mt-2 text-center w-3/4">
              Our platform offers automated seat selection to provide you with the best
              available options based on your preferences. Whether you’re attending concerts,
              sports, or theater performances, Ticket Manager simplifies the entire process,
              so you can enjoy a quick, hassle-free experience.
            </p>
          </div>
        </div>
      </BaseComponent>
    );
  }
}

export default Home;

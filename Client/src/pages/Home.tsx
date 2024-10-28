import AxiosService from '../classes/AxiosService';
import ROUTES from '../constants/routes';
import Stadium from '../components/Stadium';
import BaseComponent from '../classes/BaseComponent';
import Button from '../components/Button';
import HomeCard from '../components/HomeCard';
import { FaCalendarAlt } from "react-icons/fa";
import { LuMapPin } from "react-icons/lu";
import { MdPeople } from "react-icons/md";
import { CiCircleAlert } from "react-icons/ci";
import { Link } from 'react-router-dom';

class Home extends BaseComponent {
  private apiService: AxiosService;
  private availableZoneSeats: number[];

  constructor(props: {}) {
    super(props);
    this.apiService = AxiosService.getInstance();
    this.availableZoneSeats = [];
  }

  handleBookClick = () => {
    console.log("Start booking clicked, llamar al algoritmo aqui");
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
        <div className="flex flex-col items-center justify-center my-10 w-10/12 space-y-5">
          <h1 className="font-bold text-3xl">Welcome to Ticket Manager</h1>
          <div className="flex flex-row items-center justify-center">
            <div className="flex flex-col items-center justify-center w-1/2 h-full">
              <Stadium />
            </div>
            <div className="flex flex-col items-start justify-center w-1/2 h-full border-gray-300 border-2 rounded-lg p-10">
              <h1 className="font-bold text-xl">Automated Seat Selection</h1>
              <p className="text-gray-700 text-md">
                Find the best available options based on your preferences
              </p>
              <p className="py-7">
                Our platform offers automated seat selection to provide you with the best
                available options based on your preferences. Whether you’re attending concerts,
                sports, or theater performances, Ticket Manager simplifies the entire process,
                so you can enjoy a quick, hassle-free experience.
              </p>
              <Link className='bg-black text-white p-3 rounded-lg' to={"/booking"}>Start booking</Link>
            </div>
          </div>
          <div className="flex flex-col items-start justify-center w-full border-gray-300 border-2 rounded-lg p-10">
            <h2 className="font-bold text-2xl">Next Big Game</h2>
            <p className="text-gray-700 text-md pb-4">
              Don´t miss out the action!
            </p>
            <p>
              Local Team vs. Rival Team
            </p>
            <p>
              Date: July 15, 2023
            </p>
            <p className="pb-4">
              Time: 7:00 PM
            </p>
          </div>
          <div className="flex flex-row items-center justify-between space-x-3">
            <HomeCard title="Upcoming Events" description="Check out our calendar for all scheduled events." Icon={FaCalendarAlt} />
            <HomeCard title="Venue Information" description="Get details about our stadium and facilities." Icon={LuMapPin} />
            <HomeCard title="Group Bookings" description="Special rates and arrangements for large groups." Icon={MdPeople} />
            <HomeCard title="FAQs" description="Find answers to common questions about ticketing." Icon={CiCircleAlert} />
          </div>
        </div>
      </BaseComponent>
    );
  }
}

export default Home;

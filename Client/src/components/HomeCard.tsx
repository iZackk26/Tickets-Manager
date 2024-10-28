import { Component } from 'react';
import { IconType } from 'react-icons';

type HomeCardProps = {
  title: string;
  description: string;
  Icon: IconType;
};

class HomeCard extends Component<HomeCardProps> {
  render() {
    const { title, description, Icon } = this.props;

    return (
      <div className="flex flex-col items-start justify-center w-full h-52 px-10 py-6 border-2 border-gray-300 rounded-lg">
        <div className="flex flex-row items-center justify-start space-x-3 mb-4">
          <Icon className="text-4xl mb-2" />
          <h2 className="text-xl font-bold mb-2">{title}</h2>
        </div>
        <p className="w-3/4">{description}</p>
      </div>
    );
  }
}

export default HomeCard;

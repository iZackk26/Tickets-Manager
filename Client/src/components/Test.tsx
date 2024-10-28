import { PieChart } from '@mui/x-charts/PieChart';
import stadiumZones from './webUsageStats';
import { useNavigate } from 'react-router-dom';

export default function PieActiveArc() {
  const navigate = useNavigate();
  const handleArcClick = (event: any) => {
    const zoneIndex = Array.from(event.target.parentNode.children).indexOf(event.target);
    const zone = stadiumZones[zoneIndex];
    console.log(zone.label.toLowerCase());
  };

  return (
    <PieChart
      series={[
        {
          startAngle: 45,
          endAngle: 405,
          data: stadiumZones,
          highlightScope: { fade: 'global', highlight: 'item' },
          faded: { innerRadius: 0, additionalRadius: -30, color: 'gray' },
          arcLabel: 'label',
        },

      ]}
      slotProps={{
        legend: { hidden: true },
        pieArc: {
          onClick: handleArcClick,
          cursor: "pointer"
        }
      }}
      height={400}
    />
  );
}

import { PieChart } from '@mui/x-charts/PieChart';
import stadiumZones from './webUsageStats';
import { useNavigate } from 'react-router-dom';

function Stadium() {
  const navigate = useNavigate();

  const handleArcClick = (event: React.MouseEvent<SVGElement>) => {
    // Obtener el índice del arco clickeado
    const zoneIndex = Array.from(event.currentTarget.parentNode!.children).indexOf(event.currentTarget);
    const zone = stadiumZones[zoneIndex];

    // Navegar a /category pasando el estado de la zona seleccionada
    navigate("/category", { state: { zone: zone.label } });
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
          cursor: "pointer",
        }
      }}
      height={300}
    />
  );
}

export default Stadium;

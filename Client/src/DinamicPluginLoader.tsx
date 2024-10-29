import { useEffect, useState } from 'react';
import CreditCard from './classes/CreditCard';
import { Carousel } from "@material-tailwind/react";

export default function DinamicPluginLoader() {
  const [plugins, setPlugins] = useState<React.ComponentType[]>([]); // Almacena componentes, no objetos.

  useEffect(() => {
    const loadPlugins = async () => {
      const modules = import.meta.glob('./plugins/*.tsx');
      const loadedPlugins: React.ComponentType[] = [];

      for (const path in modules) {
        const module = await modules[path]();
        const PluginComponent = module?.default;

        // Verificar si el componente es una función y si extiende de CreditCard
        if (typeof PluginComponent === 'function') {
          // Verificar si el componente extiende de CreditCard
          const isCreditCard = (PluginComponent as any).prototype instanceof CreditCard;

          if (isCreditCard) {
            loadedPlugins.push(PluginComponent); // Guardar solo los que extienden CreditCard
          } else {
            console.warn(`The module at ${path} does not export a valid CreditCard component.`);
          }
        } else {
          console.warn(`The module at ${path} does not export a valid React component.`);
        }
      }

      setPlugins(loadedPlugins);
    };

    loadPlugins();
  }, []);

  return (
    <CarouselCustomNavigation plugins={plugins} />
  );
}

interface CarouselCustomNavigationProps {
  plugins: React.ComponentType[];
}

function CarouselCustomNavigation({ plugins }: CarouselCustomNavigationProps) {
  return (
    <Carousel
      className="rounded-xl bg-gray-900 py-8"
      navigation={({ setActiveIndex, activeIndex, length }) => (
        <div className="absolute bottom-4 left-2/4 z-50 flex -translate-x-2/4 gap-2">
          {new Array(length).fill("").map((_, i) => (
            <span
              key={i}
              className={`block h-1 cursor-pointer rounded-2xl transition-all content-[''] ${activeIndex === i ? "w-8 bg-white" : "w-4 bg-white/50"
                }`}
              onClick={() => setActiveIndex(i)}
            />
          ))}
        </div>
      )}
    >
      {plugins.map((PluginComponent, index) => (
        <div key={index} className="h-full w-full flex justify-center items-center">
          <PluginComponent /> {/* Renderizar cada tarjeta */}
        </div>
      ))}
    </Carousel>
  );
}

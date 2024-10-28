import { useEffect, useState } from 'react';
import CreditCard from './classes/CreditCard';

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
    <div>
      {plugins.map((PluginComponent, index) => (
        <div key={index} className='space-y-20'>
          <PluginComponent /> {/* Pasar props, si es necesario */}
        </div>
      ))}
    </div>
  );
}

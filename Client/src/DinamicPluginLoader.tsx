import { useEffect, useState } from 'react';
import PaymentType from './types/Payment';

export default function DinamicPluginLoader() {
  const [plugins, setPlugins] = useState<React.ComponentType<PaymentType>[]>([]); // Almacena componentes, no objetos.

  useEffect(() => {
    const loadPlugins = async () => {
      const modules = import.meta.glob('./plugins/*.tsx');
      const loadedPlugins: React.ComponentType<PaymentType>[] = [];

      for (const path in modules) {
        const module = await modules[path]();
        const PluginComponent = module?.default;

        if (typeof PluginComponent === 'function') {
          loadedPlugins.push(PluginComponent as React.ComponentType<PaymentType>);
        } else {
          console.warn(`The module at ${path} does not export a valid React component.`);
        }
      }

      setPlugins(loadedPlugins);
    };

    loadPlugins();
  }, []);

  const samplePaymentData: PaymentType = {
    cardNumber: 1234567812345678,
    cardHolderName: 'John Doe',
    expirationDate: new Date('2024-12-31'),
    cvv: 123,
  };

  return (
    <div>
      {plugins.map((PluginComponent, index) => (
        <div key={index}>
          <PluginComponent {...samplePaymentData} /> {/* Pasar props del tipo PaymentType */}
        </div>
      ))}
    </div>
  );
}

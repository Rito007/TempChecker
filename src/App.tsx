import "./App.css"
import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Slider } from "@/components/ui/slider";

export default function App() {
  const [temperature, setTemperature] = useState<number | null>(null);
  const [threshold, setThreshold] = useState<number>(40);
  const [activated, setActivated] = useState(true);
  const botao = useRef(null);

  useEffect(() => {
    // Carrega o limite inicial ao montar
    invoke<number>("get_limit_command").then(setThreshold);

    // Atualiza a temperatura periodicamente
    const interval = setInterval(() => {
      invoke<number>("get_temperature_command").then((temp) => setTemperature(temp));
    }, 5000);

    return () => clearInterval(interval);
  }, []);

  const handleThresholdChange = (value: number[]) => {
    const newLimit = value[0];
    setThreshold(newLimit);
    invoke("set_limit_command", { newLimit }); // enviar para backend
  };

  const handleOnOff = () => {
    setActivated(!activated);
    // aqui podes adicionar lógica para pausar o monitor, se necessário
  };

  return (
    <div className="min-h-screen bg-zinc-900 text-white p-6 flex items-center justify-center">
      <div className="bg-zinc-800/60 backdrop-blur-md shadow-xl rounded p-6 w-full max-w-sm space-y-6 border border-zinc-700">

        {/* Temperatura Atual */}
        <Card className="bg-zinc-800 rounded-xl shadow-sm border border-zinc-700">
          <CardHeader>
            <CardTitle className="text-base text-white">🌡️ Temperatura</CardTitle>
          </CardHeader>
          <CardContent className="text-center text-white">
            <div className="text-5xl font-semibold tracking-tight">
              {temperature !== null ? `${temperature.toFixed(1)}°C` : "--"}
            </div>
          </CardContent>
        </Card>

        {/* Limite de Temperatura */}
        <Card className="bg-zinc-800 rounded-xl shadow-sm border border-zinc-700">
          <CardHeader>
            <CardTitle className="text-base text-white">🎚️ Limite de Alerta</CardTitle>
          </CardHeader>
          <CardContent>
            <Slider
              min={30}
              max={90}
              step={1}
              value={[threshold]}
              onValueChange={handleThresholdChange}
              className="w-full"
            />
            <div className="text-center mt-2 text-sm text-zinc-400">
              Alerta acima de <strong>{threshold}°C</strong>
            </div>
          </CardContent>
        </Card>

        {/* Botão de Ligar/Desligar */}
        <Button
          ref={botao}
          onClick={handleOnOff}
          className={`w-full py-2 text-sm font-medium rounded-xl shadow-md transition-colors ${
            activated ? "bg-green-600 hover:bg-green-500" : "bg-red-600 hover:bg-red-500"
          }`}
        >
          {activated ? "Ligado" : "Desligado"}
        </Button>
      </div>
    </div>
  );
}

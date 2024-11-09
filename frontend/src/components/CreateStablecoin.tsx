import React, { useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { useProgram } from '../utils/program';

export const CreateStablecoin: React.FC = () => {
  const { connected } = useWallet();
  const { program } = useProgram();
  
  const [name, setName] = useState('');
  const [symbol, setSymbol] = useState('');
  const [currency, setCurrency] = useState('USD');
  const [iconUrl, setIconUrl] = useState('');
  
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!connected || !program) return;
    
    try {
      // Create stablecoin logic here using program
      const tx = await program.methods
        .createStablecoin(name, symbol, currency, iconUrl)
        .rpc();
      
      console.log("Transaction signature", tx);
    } catch (error) {
      console.error("Error creating stablecoin:", error);
    }
  };
  
  return (
    <div className="bg-white p-6 rounded-lg shadow-lg">
      <h2 className="text-2xl font-bold mb-4">Create New Stablecoin</h2>
      
      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700">
            Name
          </label>
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm"
            required
          />
        </div>
        
        {/* Add similar input fields for symbol, currency, and icon URL */}
        
        <button
          type="submit"
          disabled={!connected}
          className="bg-blue-500 text-white px-4 py-2 rounded-md"
        >
          Create Stablecoin
        </button>
      </form>
    </div>
  );
}; 
import React from 'react';
import { WalletProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets';
import { Navigation } from './components/Navigation';
import { CreateStablecoin } from './components/CreateStablecoin';
import { StablecoinList } from './components/StablecoinList';
import { MintRedeem } from './components/MintRedeem';

require('@solana/wallet-adapter-react-ui/styles.css');

const wallets = [new PhantomWalletAdapter()];

function App() {
  return (
    <WalletProvider wallets={wallets} autoConnect>
      <WalletModalProvider>
        <div className="min-h-screen bg-gray-100">
          <Navigation />
          <main className="container mx-auto px-4 py-8">
            <CreateStablecoin />
            <StablecoinList />
            <MintRedeem />
          </main>
        </div>
      </WalletModalProvider>
    </WalletProvider>
  );
}

export default App; 
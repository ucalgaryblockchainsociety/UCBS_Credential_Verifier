import { Noto_Sans } from 'next/font/google';
import { Header } from '../components/Header';
import Link from 'next/link'; // Import Link from next/link for navigation
import Login from './login';

const notoSans = Noto_Sans({
    weight: ['300', '400', '500', '600', '700'],
    subsets: ['latin'],
});

export default function Home() {
    return (
        <div className={`bg-[#0B161E] ${notoSans.className} text-white flex flex-col items-center justify-center min-h-screen`}>
            <Header />
            <Login />
         
    
            
            <div className="space-y-4">

               
            </div>
        </div>
    );
}
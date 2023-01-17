import './App.css';
import 'bootstrap/dist/css/bootstrap.min.css';
import {Alert, Button, Form, FormControl, FormLabel} from "react-bootstrap";
import {useEffect, useState} from "react";
import {ethers} from "ethers";
import contrAbi from "./utils/UsersPublicHashes.json"
import axios from "axios";

function App() {

    // Wallet connection begins here

    let [currentAccount, setCurrentAccount] = useState();
    let [verified, setVerified] = useState(false);
    let [wrongPassword, setWrongPassword] = useState(false);



    const checkIfWalletIsConnected = async () =>{
        try{
            const { ethereum } = window;

            if(!ethereum){
                console.log("Use Metamask!");
            } else{
                console.log("Ethereum object found", ethereum);
            }

            const accounts = await ethereum.request({method: 'eth_accounts'});

            if(accounts !== 0){
                const account = accounts[0];
                console.log("Found an authorized account ", account);
                setCurrentAccount(account);
                if (!contract) {
                    setContract(await getContract())
                }
            } else{
                console.log("Could not find an authorized account");
            }
        } catch(error){
            console.log(error);
        }
    }

    const connectWallet = async () =>{
        try{
            const { ethereum } = window;

            if(!ethereum){
                alert("Use Metamask!");
            } else{
                const accounts = await ethereum.request({ method: 'eth_requestAccounts'});
                console.log("Account connected ", accounts[0]);

                setCurrentAccount(accounts[0]);
                if (!contract) {
                    setContract(await getContract())
                }
            }
        } catch(error){
            console.log(error);
        }
    }

    useEffect( () =>{

        checkIfWalletIsConnected();

    }, []);

    // Wallet connection finishes here

    let [contract, setContract] = useState();

    const getContract = async () => {
        const { ethereum } = window;
        const provider = new ethers.providers.Web3Provider(ethereum);
        const signer = provider.getSigner();
        const address = "0x9715Bd6563E8b875E569e91A435c00B83d2D5035";

        return new ethers.Contract(address, contrAbi.abi, signer);
    }

    const [createPass, setCreatePass] = useState("");
    const [verifyPass, setVerifyPass] = useState("");
    const createAcc = async () => {
        const response = await axios
            .post("http://127.0.0.1:8080/password/create",
            {
                walletAddress: currentAccount.toString().trim(),
                password: createPass.trim(),
            })
            .then(async data => {
                if (contract) {
                    contract.addUserHash(data.data)
                        .catch(err => {
                            alert("This user already exists. Log in with your password")
                        })
                }
            });
    }
    const verifyAcc = async () => {
        let hash;
        if (contract) {
           hash = await contract.getUserHash()
               .catch(err => {
                   alert("This user doesn't exist. Sign up first")
               })
        }
        if (hash) {
            const response = await axios
                .post("http://127.0.0.1:8080/password/verify",
                    {
                        walletAddress: currentAccount.toString().trim(),
                        password: verifyPass.trim(),
                        pubHash: hash.toString().trim(),
                    })
                .then(async data => {
                    console.log("The data from the method: ", data)
                    if (data.data === true) {
                        setWrongPassword(false)
                        setVerified(true)
                    }
                    if (data.data === false) {
                        setVerified(false)
                        setWrongPassword(true)
                    }
                });
        }
    }
  return (
    <div className="App">
        {!currentAccount && (
            <Button className="generalButton" onClick={connectWallet}>
                Connect Wallet
            </Button>
        )}
        <Form>
            <FormLabel>Type your password to create an account</FormLabel>
            <FormControl onChange={e => setCreatePass(e.target.value)}/>
        </Form>
      <Button variant="primary" onClick={createAcc}>Create account</Button>
        <Form>
            <FormLabel>Type your password to log in</FormLabel>
            <FormControl onChange={e => setVerifyPass(e.target.value)}/>
        </Form>
      <Button variant="success" onClick={verifyAcc}>Verify password</Button>
      {/*</header>*/}
        {verified && (
            <Alert variant="success">
                You have successfully logged in
            </Alert>
        )}
        {wrongPassword && (
            <Alert variant="danger">
                The password is incorrect
            </Alert>
        )}
    </div>
  );
}

export default App;

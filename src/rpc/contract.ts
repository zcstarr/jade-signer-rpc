

class SmartContract {

   constructor() {

  }

  buyFurby() {
    const abi = buyFurby.data.abi;
    const hash = signer.signTransaction(abi);
    return ethClient.sendTransaction(hash);
  }

}
/// The Web Browser


const smartContract = window.SmartContract;
smartContract.buyFurby();

// Alternative
class SmartContract {

   constructor() {

  }

  buyFurby() {
    const abi = buyFurby.data.abi;
    return hash = signer.signTransaction(abi);
  }

}
// The Web Browser

const smartContract = window.SmartContract;
smartContract.buyFurby();



// WebBrowser

// has all the methods of the smart contract now
// generator generates the FurbyContract from the abi
const ethClient = new ethClient();
const signer = new JadeSigner( ...{... transport: 8200});
const furbyContract = new FurbyContract(contractAddress);
/// returns transaction class
const txClassInstance = furbyContract.buyFurby();
// What happens here
/***
 * 0. signer service is running
 * 1. signer ui is running
 * 1. signer ui discovers signer via the service runner and requests a specific version
 * 1. signer ui has to register itself for a callback for signer
 * 2. user calls sign with a valid transaction to the RPC signer server  
 * 3. signer server calls the UI through the early registration method
 * 4. signer ui is like yo we got a sign request give me a password
 * 5. user enters password
 * 6. signer ui sends unencrypted password to signer 
 * 7. signer is then like hey got the keys here's the signed transaction
 * 8. the dApp returns signedTx data  
 */
const signedTX = await signer.sign(txClassInstance.toJSON())


/***
 * WHAT IF WE WERE REMOTE!!!!!!!!!!!
 * 0. signer service is running on desktop A
 * 1. signer ui is running on desktop B
 * 1. signer ui discovers signer via the service runner and requests a specific version
 * 1. signer ui has to register itself for a callback for signer
 * 2. user calls sign with a valid transaction to the RPC signer server  
 * 3. signer server calls the UI through the early registration method
 * 4. signer ui is like yo we got a sign request give me a password
 * 5. user enters password
 * 6. signer ui sends unencrypted password to signer 
 * 7. signer is then like hey got the keys here's the signed transaction
 * 8. the dApp returns signedTx data  
 */
const signedTX = await signer.sign(txClassInstance.toJSON()

const txHash = await ethClient.sendRawTransaction(signedTX);



// The easy look is this
const superFurbyContract = SuperFurbyContract(contractAddress, signer, ethClient);
const txHash = await superFurbyContract.buyFurby();









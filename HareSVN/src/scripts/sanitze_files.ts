import { invoke } from "@tauri-apps/api";

export async function sanitize_files(repo:string){
   let files:Array<{status:string, fileName:string}> = []
   let temp:Array<string>= await invoke("status", { name:repo });
    console.log(temp)
    temp.map((file:string)=>{
      let temp= file.split(" ").filter((character)=> character != "")
      if (temp[0].charAt(temp[0].length-1) == "?" && temp[temp.length-1] != "."){
        files.push({status: "Untracked", fileName: temp[temp.length-1]})
      }
      else if (temp[0].charAt(temp[0].length-1) == "A" && temp[temp.length-1] != "."){
        files.push({status: "Added", fileName: temp[temp.length-1]})
      }
      else if (temp[0].charAt(temp[0].length-1) == "M" && temp[temp.length-1] != "."){
        files.push({status: "Modified", fileName: temp[temp.length-1]})
      }
      else if (temp[0].charAt(temp[0].length-1) == "!" && temp[temp.length-1] != "."){
        files.push({status: "Missing", fileName: temp[temp.length-1]})
      }
      else if(temp[temp.length-1] != "." && temp[temp.length-1] != ""){
        files.push({status: "Up To Date", fileName: temp[temp.length-1]})
      }
      else if (temp[0].charAt(temp[0].length-1) == "D" && temp[temp.length-1] != "."){
        files.push({status: "Deleting", fileName: temp[temp.length-1]})
      }
      return file.split(" ").filter((character)=> character != "")
    })
    return files
}
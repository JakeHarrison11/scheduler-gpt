# PA#1 - The ChatGPT Scheduler

## The Conversation
[OpenCode session link](https://opncd.ai/share/XLw8y6yM)
This conversation was carried out by Jake, but both members contributed to the prompt. We decided against having multiple conversations since AI works best in small threads where it can review all the context. Besides, the initial prompt already solved most of the problem with only some additional help required to polish the result. We also had the AI write a simple bash script to verify the output.

## Evaluation
**Correctness: Does the code perform the intended task correctly? Are there any bugs or errors that need to be fixed?**  
The code performs the task correctly with no bugs. All formatting is also in-line with expectations and nothing needs to be fixed.

**Efficiency: Is the code efficient and avoid unnecessary computations or data structures? Can the code be optimized for better performance?**  
The code is efficient and avoids unnecessary computations or data structures. These algorithms are simple and widely-known, so the AI already implemented them in a way that I don't see how can be made more efficient.


**Readability: Is the code well-organized, well-documented, and easy to understand? Does the code follow best practices such as using meaningful variable names, avoiding code duplication, and use a consistent coding style between prompts?**  
The code mostly follows best practices with naming conventions, except the one variable "processcount" which is in no conventional casing standard. There are also no comments throughout the code, but it is readable even for those not familiar with Rust particularly. Other than that, the coding style itself is consistent, partially due to the small amount of prompts used.


**Completeness: Does the code handle edge cases and error conditions appropriately? Is the code flexible enough to handle different input data and scenarios? What happens when you don't feed it an input file or a malformed input file? Does the code account for race conditions (i.e. when two processes can technically be scheduled at the same time)?**  
The main function automatically deals with no input file or malformed input data and automatically notifies when it encounters any errors with producing output. The code also properly accounts for race conditions in each of the algorithms it implements.


**Learning Assistance: Did you need AI assistance or additional support in learning the language?  If so, please provide your prompts and a summary of the responses.  Some of you may have had enough success using AI that maybe you felt you didn't need to learn Rust to get good results or perhaps the code was clear enough that you didn't feel the need to use additional resources to understand it.  If this is the case, please describe.  Do you feel that it's possible to use AI to code apps in languages you don't know or understand?  What limitations do you foresee?**    
We did not require any AI assistance in learning the language and any questions had could easily be answered by the documentation or a Google search. In fact, we barely had to read and understand the code (hence the lack of comments) due to how quickly the AI properly created the code with only formatting errors that needed to be corrected that the AI also fixed on its own.
I don't believe it's possible to create an entire app in a language you don't know with AI unless you're simply asking for conversion from something you already wrote or you have someone else who can check the code the AI outputs due to being unable to see minor edge cases as the app's code gets too large for the AI to process at once.


**Overall Quality and Final Recommendation: Based on the above criteria, how would you rate the overall quality of the code? Would you recommend any changes or improvements to make the code more effective or efficient? How would you rate your overall experience writing code using the assistance of an AI? Was it easier or harder than you expected? What did you learn through the process? What would you do differently if you had to write code via AI again?**  
The quality of the code itself is good. As stated previously, it lacks comments, but it performs the task it's meant to and a person could easily add comments given how clear the code already is. 

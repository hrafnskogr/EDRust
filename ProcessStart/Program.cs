// Code adapted from www.fluxbytes.com/csharp/how-to-know-if-a-process-exited-or-started-using-events-in-c/

using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Management;

namespace ProcessStart
{
    class Program
    {
        static ManagementEventWatcher processStartEvent = new ManagementEventWatcher("SELECT * FROM Win32_ProcessStartTrace");
        static ManagementEventWatcher processStopEvent = new ManagementEventWatcher("SELECT * FROM Win32_ProcessStopTrace");

        static void Main(string[] args)
        {
            processStartEvent.EventArrived += new EventArrivedEventHandler(processStartEvent_EventArrived);
            processStartEvent.Start();
            //processStopEvent.EventArrived += new EventArrivedEventHandler(processStopEvent_EventArrived);
            //processStopEvent.Start();

            while(true){}
        }

        static void processStartEvent_EventArrived(object sender, EventArrivedEventArgs e)
        {
            string processName = e.NewEvent.Properties["ProcessName"].Value.ToString();
            string processID = Convert.ToInt32(e.NewEvent.Properties["ProcessID"].Value).ToString();

            Console.WriteLine(processName + "|" + processID);
            //Console.WriteLine(processID);
        }

        static void processStopEvent_EventArrived(object sender, EventArrivedEventArgs e)
        {
            string processName = e.NewEvent.Properties["ProcessName"].Value.ToString();
            string processID = Convert.ToInt32(e.NewEvent.Properties["ProcessID"].Value).ToString();

            Console.WriteLine("Process stopped. Name: " + processName + " | ID: " + processID);
        }
    }
}

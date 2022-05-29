# Noted2Xero core library   
This command line takes exported CSVs from the Noted cloud application to a CSV for importing invoices into Xero

## Call by 

Can run both under nightly or standard rust release. This is a simple commandline that will look for a Noted CSV in the resources/notedfolder in the folder it is running from. It also expects the xerofolder and donefolder there. (See the make init for details) You need to pass a parameter that is the first free invoice number available on the Xero account you want to import your invoices to. For instance if INV-2999 is your latest invoice then:

```bash
./noted2xero_cli 3000
```

See make run_cli for how it runs from this project.